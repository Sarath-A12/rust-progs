// Won't work without pub
// By default functions inside modules... it's private
pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}