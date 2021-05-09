use core::{cell::UnsafeCell, fmt};
use cortex_a::{asm, barrier, regs::*};
use register::InMemoryRegister;

global_asm!(include_str!("exception.s"));

/// Wrapper struct for memory copy of SPSR_EL1.
#[repr(transparent)]
struct SpsrEL1(InMemoryRegister<u64, SPSR_EL1::Register>);

/// The exception context as it is stored on the stack on exception entry.
#[repr(C)]
struct ExceptionContext {
    /// General Purpose Registers.
    gpr: [u64; 30],

    /// The link register, aka x30.
    lr: u64,

    /// Exception link register. The program counter at the time the exception happened.
    elr_el1: u64,

    /// Saved program status.
    spsr_el1: SpsrEL1,
}

/// Wrapper struct for pretty printing ESR_EL1.
struct EsrEL1;

/// Prints verbose information about the exception and then panics.
fn default_exception_handler(e: &ExceptionContext) {
    crate::print!(
        "\n\nCPU Exception!\n\
         FAR_EL1: {:#018x}\n\
         {}",
        FAR_EL1.get(),
        e
    );
    panic!()
}

#[no_mangle]
unsafe extern "C" fn current_elx_synchronous(e: &mut ExceptionContext) {
    let far_el1 = FAR_EL1.get();

    // This catches the demo case for this tutorial. If the fault address happens to be 8 GiB,
    // advance the exception link register for one instruction, so that execution can continue.
    if far_el1 == 8 * 1024 * 1024 * 1024 {
        e.elr_el1 += 4;

        asm::eret()
    }

    default_exception_handler(e);
}

/// Human readable SPSR_EL1.
#[rustfmt::skip]
impl fmt::Display for SpsrEL1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Raw value.
        writeln!(f, "SPSR_EL1: {:#010x}", self.0.get())?;

        let to_flag_str = |x| -> _ {
            if x { "Set" } else { "Not set" }
         };

        writeln!(f, "      Flags:")?;
        writeln!(f, "            Negative (N): {}", to_flag_str(self.0.is_set(SPSR_EL1::N)))?;
        writeln!(f, "            Zero     (Z): {}", to_flag_str(self.0.is_set(SPSR_EL1::Z)))?;
        writeln!(f, "            Carry    (C): {}", to_flag_str(self.0.is_set(SPSR_EL1::C)))?;
        writeln!(f, "            Overflow (V): {}", to_flag_str(self.0.is_set(SPSR_EL1::V)))?;

        let to_mask_str = |x| -> _ {
            if x { "Masked" } else { "Unmasked" }
        };

        writeln!(f, "      Exception handling state:")?;
        writeln!(f, "            Debug  (D): {}", to_mask_str(self.0.is_set(SPSR_EL1::D)))?;
        writeln!(f, "            SError (A): {}", to_mask_str(self.0.is_set(SPSR_EL1::A)))?;
        writeln!(f, "            IRQ    (I): {}", to_mask_str(self.0.is_set(SPSR_EL1::I)))?;
        writeln!(f, "            FIQ    (F): {}", to_mask_str(self.0.is_set(SPSR_EL1::F)))?;

        write!(f, "      Illegal Execution State (IL): {}",
            to_flag_str(self.0.is_set(SPSR_EL1::IL))
        )?;

        Ok(())
    }
}

/// Human readable print of the exception context.
impl fmt::Display for ExceptionContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "ELR_EL1: {:#018x}", self.elr_el1)?;
        writeln!(f, "{}", self.spsr_el1)?;
        writeln!(f)?;
        writeln!(f, "General purpose register:")?;

        #[rustfmt::skip]
        let alternating = |x| -> _ {
            if x % 2 == 0 { "   " } else { "\n" }
        };

        // Print two registers per line.
        for (i, reg) in self.gpr.iter().enumerate() {
            write!(f, "      x{: <2}: {: >#018x}{}", i, reg, alternating(i))?;
        }
        write!(f, "      lr : {:#018x}", self.lr)?;

        Ok(())
    }
}

/// Init exception handling by setting the exception vector base address register.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
/// - The vector table and the symbol `__exception_vector_table_start` from the linker script must
///   adhere to the alignment and size constraints demanded by the ARMv8-A Architecture Reference
///   Manual.
pub unsafe fn handling_init() {
    // Provided by exception.S.
    extern "Rust" {
        static __EXCEPTION_VECTOR_START: UnsafeCell<()>;
    }

    unsafe {
        VBAR_EL1.set(__EXCEPTION_VECTOR_START.get() as u64);
        barrier::isb(barrier::SY);
    }
}

/// The processing element's current privilege level.
pub fn current_privilege_level() -> &'static str {
    let el = CurrentEL.read_as_enum(CurrentEL::EL);
    match el {
        Some(CurrentEL::EL::Value::EL2) => "EL2",
        Some(CurrentEL::EL::Value::EL1) => "EL1",
        Some(CurrentEL::EL::Value::EL0) => "EL0",
        _ => "Unknown",
    }
}
