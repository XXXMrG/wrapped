use icrate::{
    block2::ConcreteBlock,
    objc2::rc::Id,
    AppKit::{
        NSRunningApplication, NSWorkspace, NSWorkspaceApplicationKey,
        NSWorkspaceDidActivateApplicationNotification,
    },
    Foundation::{NSDictionary, NSNotification, NSRunLoop},
};
use std::mem;
use std::ptr::NonNull;

fn main() {
    println!("Hello, world!");
    unsafe {
        let shared = NSWorkspace::sharedWorkspace();
        println!("{:?}", shared.notificationCenter());

        let notify_center = shared.notificationCenter();

        let block = ConcreteBlock::new(|notification: NonNull<NSNotification>| {
            // println!("{:?}", notification);
            let ptr = notification.as_ref();
            let user_info: Id<NSDictionary> = ptr.userInfo().unwrap();

            // TODO: find a better way to transform type.
            let app: &NSRunningApplication =
                mem::transmute(user_info.get(NSWorkspaceApplicationKey).unwrap());

            println!("{:?}", app.localizedName().unwrap());
        })
        .copy();

        notify_center.addObserverForName_object_queue_usingBlock(
            Some(NSWorkspaceDidActivateApplicationNotification),
            None,
            None,
            &block,
        );

        // TODO: find a better way to start main thread
        NSRunLoop::mainRunLoop().run();
    }
}
