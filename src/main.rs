use icrate::{
    block2::ConcreteBlock,
    AppKit::{NSWorkspace, NSWorkspaceDidDeactivateApplicationNotification},
    Foundation::NSNotification,
};
use std::ptr::NonNull;

fn main() {
    println!("Hello, world!");
    unsafe {
        let shared = NSWorkspace::sharedWorkspace();
        println!("{:?}", shared.notificationCenter());

        let notify_center = shared.notificationCenter();

        let block = ConcreteBlock::new(|notification: NonNull<NSNotification>| {
            println!("{:?}", notification);
        })
        .copy();

        notify_center.addObserverForName_object_queue_usingBlock(
            Some(NSWorkspaceDidDeactivateApplicationNotification),
            None,
            None,
            &block,
        );
    }
}
