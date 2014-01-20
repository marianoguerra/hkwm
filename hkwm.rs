extern mod xlib;
use std::ptr;
use std::option::Option;
use xlib::xlib;
use xlib::{Display, XSelectInput, XOpenDisplay, Window, XDefaultRootWindow};
use xlib::{XNextEvent, Union__XEvent};


// taken from X.h
//static KeyPress:i32 = 2;
static KeyRelease:i32 = 3;
/*static ButtonPress:i32 = 4;
static ButtonRelease:i32 = 5;
static MotionNotify:i32 = 6;
static EnterNotify:i32 = 7;
static LeaveNotify:i32 = 8;
static FocusIn:i32 = 9;
static FocusOut:i32 = 10;
static KeymapNotify:i32 = 11;
static Expose:i32 = 12;
static GraphicsExpose:i32 = 13;
static NoExpose:i32 = 14;
static VisibilityNotify:i32 = 15;
static CreateNotify:i32 = 16;
static DestroyNotify:i32 = 17;
static UnmapNotify:i32 = 18;
static MapNotify:i32 = 19;
*/
static MapRequest:i32 = 20;
/*
static ReparentNotify:i32 = 21;
static ConfigureNotify:i32 = 22;
static ConfigureRequest:i32 = 23;
static GravityNotify:i32 = 24;
static ResizeRequest:i32 = 25;
static CirculateNotify:i32 = 26;
static CirculateRequest:i32 = 27;
static PropertyNotify:i32 = 28;
static SelectionClear:i32 = 29;
static SelectionRequest:i32 = 30;
static SelectionNotify:i32 = 31;
static ColormapNotify:i32 = 32;
static ClientMessage:i32 = 33;
static MappingNotify:i32 = 34;
static GenericEvent:i32 = 35;
static LASTEvent:i32 = 36; // must be bigger than any event #
*/

fn getDisplay() -> Option<*mut Display> {
    unsafe {
        let null = ptr::null();
        let display = XOpenDisplay(null);

        if (ptr::is_null(display)) {
            return None
        } else {
            return Some(display);
        }

    }
}

fn getRootWindow(display: *mut Display) -> Window {
    unsafe {
        return XDefaultRootWindow(display);
    }
}

fn setSelectInput(display: *mut Display, window: Window) {
    unsafe {
        let SubstructureNotifyMask = 1<<19;
        let SubstructureRedirectMask = 1<<20;
        let XSelectInputMask = SubstructureNotifyMask|SubstructureRedirectMask;
        XSelectInput(display, window, XSelectInputMask);
    }
}

fn main() {
    let maybeDisplay = getDisplay();
    let Mod4Mask = (1<<16);
    let GrabModeAsync = 1;

    match maybeDisplay {
        Some(display) => {
            println!("Got display {}", display);
            let root = getRootWindow(display);
            println!("Got root window {}", root);

            setSelectInput(display, root);
            println!("select input set!");

            unsafe {
                let mut wa = xlib::XSetWindowAttributes {
                    background_pixmap: 0,
                    background_pixel: 0,
                    border_pixmap: 0,
                    border_pixel: 0,
                    bit_gravity: 0,
                    win_gravity: 0,
                    backing_store: 0,
                    backing_planes: 0,
                    backing_pixel: 0,
                    save_under: 0,
                    event_mask: 0,
                    do_not_propagate_mask: 0,
                    override_redirect: 0,
                    colormap: 0,
                    cursor: 0,
                };
                wa.cursor = xlib::XCreateFontCursor(display, 68 /*XC_left_ptr*/);
                xlib::XChangeWindowAttributes(display, root,
                    (1<<14) /*CWCursor*/, &mut wa);
                xlib::XGrabKey(display, 53, Mod4Mask, root, 1, GrabModeAsync,
                    GrabModeAsync);
                xlib::XGrabKey(display, 45, Mod4Mask, root, 1, GrabModeAsync,
                    GrabModeAsync);

                let mut xevent = Union__XEvent{ data: [0, ..24] };
                loop {
                    XNextEvent(display, ptr::to_mut_unsafe_ptr(&mut xevent));
                    let eventType = *xevent._type();

                    match eventType {
                        MapRequest => {
                            let event = *xevent.xmaprequest();
                            xlib::XMapWindow(display, event.window);
                            xlib::XSetInputFocus(display, event.window,
                                1 /*RevertToPointerRoot*/, 0 /*CurrentTime*/);
                            xlib::XMoveResizeWindow(display, event.window, 0, 0,
                                1920, 1080);
                        }
                        KeyRelease => {
                            let event = *xevent.xkey();
                            println!("Key {}", event.keycode);
                            if (event.keycode == 45) {
                                    let mut focus: Window = 0;
                                    let mut revert: i32 = 0;
                                    xlib::XGetInputFocus(display, &mut focus,
                                        &mut revert);
                                    xlib::XDestroyWindow(display, focus);
                            }
                        }
                        _ => {
                            println!("Unknown Event Type {}", eventType);
                        }
                    }
                }
            }
        }
        None => println!("Error getting display")
    }
}
