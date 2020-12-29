use crate::console::kprintln;
use core::panic::PanicInfo;

const MESSAGE:&str ="           (
    (      )     )
      )   (    (
     (          `
 .-\"\"^\"\"\"^\"\"^\"\"\"^\"\"-.
(//\\//\\//\\//\\//\\//\\//)
~\\^^^^^^^^^^^^^^^^^^/~
  `================`

 The pi is overdone.

---------- PANIC ---------- ";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("{}", MESSAGE);
    if let Some(location) = info.location() {
            kprintln!("panic occurred in file '{}' at line {} col {}",
                location.file(),
                location.line(),
                location.column()
            );
        } else {
            kprintln!("panic occurred but can't get location information...");
        }
    if let Some(s) = info.payload().downcast_ref::<&str>() {
            kprintln!("panic occurred: {:?}", s);
    }
    loop {}
}
