#[allow(dead_code)];
pub static X_PROTOCOL:i32 = 11; /* current protocol version */
pub static X_PROTOCOL_REVISION:i32 = 0; /* current minor version */

/* Resources */
pub static None:i64 = 0;	/* universal null resource or null atom */
pub static ParentRelative:i64 = 1;	/* background pixmap in CreateWindow and ChangeWindowAttributes */
pub static CopyFromParent:i64 = 0;	/* border pixmap in CreateWindow
				       and ChangeWindowAttributes
				   special VisualID and special window
				       class passed to CreateWindow */

pub static PointerWindow:i64 = 0;	/* destination window in SendEvent */
pub static InputFocus:i64 = 1;	/* destination window in SendEvent */

pub static PointerRoot:i64 = 1;	/* focus window in SetInputFocus */

pub static AnyPropertyType:i64 = 0;	/* special Atom, passed to GetProperty */

pub static AnyKey:i64 = 0;	/* special Key Code, passed to GrabKey */

pub static AnyButton:i64 = 0;	/* special Button Code, passed to GrabButton */

pub static AllTemporary:i64 = 0;	/* special Resource ID passed to KillClient */

pub static CurrentTime:u64 = 0;	/* special Time */

pub static NoSymbol:i64 = 0;	/* special KeySym */

/***************************************************************** 
 * EVENT DEFINITIONS 
 *****************************************************************/

/* Input Event Masks. Used as event-mask window attribute and as arguments
   to Grab requests.  Not to be confused with event names.  */

pub static NoEventMask:i64 = 0;
pub static KeyPressMask:i64 = (1<<0);
pub static KeyReleaseMask:i64 = (1<<1);
pub static ButtonPressMask:i64 = (1<<2);
pub static ButtonReleaseMask:i64 = (1<<3);
pub static EnterWindowMask:i64 = (1<<4);
pub static LeaveWindowMask:i64 = (1<<5);
pub static PointerMotionMask:i64 = (1<<6);
pub static PointerMotionHintMask:i64 = (1<<7);
pub static Button1MotionMask:i64 = (1<<8);
pub static Button2MotionMask:i64 = (1<<9);
pub static Button3MotionMask:i64 = (1<<10);
pub static Button4MotionMask:i64 = (1<<11);
pub static Button5MotionMask:i64 = (1<<12);
pub static ButtonMotionMask:i64 = (1<<13);
pub static KeymapStateMask:i64 = (1<<14);
pub static ExposureMask:i64 = (1<<15);
pub static VisibilityChangeMask:i64 = (1<<16);
pub static StructureNotifyMask:i64 = (1<<17);
pub static ResizeRedirectMask:i64 = (1<<18);
pub static SubstructureNotifyMask:i64 = (1<<19);
pub static SubstructureRedirectMask:i64 = (1<<20);
pub static FocusChangeMask:i64 = (1<<21);
pub static PropertyChangeMask:i64 = (1<<22);
pub static ColormapChangeMask:i64 = (1<<23);
pub static OwnerGrabButtonMask:i64 = (1<<24);

/* Event names.  Used in "type" field in XEvent structures.  Not to be
confused with event masks above.  They start from 2 because 0 and 1
are reserved in the protocol for errors and replies. */

pub static KeyPress:i32 = 2;
pub static KeyRelease:i32 = 3;
pub static ButtonPress:i32 = 4;
pub static ButtonRelease:i32 = 5;
pub static MotionNotify:i32 = 6;
pub static EnterNotify:i32 = 7;
pub static LeaveNotify:i32 = 8;
pub static FocusIn:i32 = 9;
pub static FocusOut:i32 = 10;
pub static KeymapNotify:i32 = 11;
pub static Expose:i32 = 12;
pub static GraphicsExpose:i32 = 13;
pub static NoExpose:i32 = 14;
pub static VisibilityNotify:i32 = 15;
pub static CreateNotify:i32 = 16;
pub static DestroyNotify:i32 = 17;
pub static UnmapNotify:i32 = 18;
pub static MapNotify:i32 = 19;
pub static MapRequest:i32 = 20;
pub static ReparentNotify:i32 = 21;
pub static ConfigureNotify:i32 = 22;
pub static ConfigureRequest:i32 = 23;
pub static GravityNotify:i32 = 24;
pub static ResizeRequest:i32 = 25;
pub static CirculateNotify:i32 = 26;
pub static CirculateRequest:i32 = 27;
pub static PropertyNotify:i32 = 28;
pub static SelectionClear:i32 = 29;
pub static SelectionRequest:i32 = 30;
pub static SelectionNotify:i32 = 31;
pub static ColormapNotify:i32 = 32;
pub static ClientMessage:i32 = 33;
pub static MappingNotify:i32 = 34;
pub static GenericEvent:i32 = 35;
pub static LASTEvent:i32 = 36; /* must be bigger than any event # */


/* Key masks. Used as modifiers to GrabButton and GrabKey, results of QueryPointer,
   state in various key-, mouse-, and button-related events. */

pub static ShiftMask:u32 = (1<<0);
pub static LockMask:u32 = (1<<1);
pub static ControlMask:u32 = (1<<2);
pub static Mod1Mask:u32 = (1<<3);
pub static Mod2Mask:u32 = (1<<4);
pub static Mod3Mask:u32 = (1<<5);
pub static Mod4Mask:u32 = (1<<6);
pub static Mod5Mask:u32 = (1<<7);

/* modifier names.  Used to build a SetModifierMapping request or
   to read a GetModifierMapping request.  These correspond to the
   masks defined above. */
pub static ShiftMapIndex:i32 = 0;
pub static LockMapIndex:i32 = 1;
pub static ControlMapIndex:i32 = 2;
pub static Mod1MapIndex:i32 = 3;
pub static Mod2MapIndex:i32 = 4;
pub static Mod3MapIndex:i32 = 5;
pub static Mod4MapIndex:i32 = 6;
pub static Mod5MapIndex:i32 = 7;

/* button masks.  Used in same manner as Key masks above. Not to be confused
   with button names below. */

pub static Button1Mask:i32 = (1<<8);
pub static Button2Mask:i32 = (1<<9);
pub static Button3Mask:i32 = (1<<10);
pub static Button4Mask:i32 = (1<<11);
pub static Button5Mask:i32 = (1<<12);

pub static AnyModifier:i32 = (1<<15);  /* used in GrabButton, GrabKey */


/* button names. Used as arguments to GrabButton and as detail in ButtonPress
   and ButtonRelease events.  Not to be confused with button masks above.
   Note that 0 is already defined above as "AnyButton".  */

pub static Button1:i32 = 1;
pub static Button2:i32 = 2;
pub static Button3:i32 = 3;
pub static Button4:i32 = 4;
pub static Button5:i32 = 5;

/* Notify modes */

pub static NotifyNormal:i32 = 0;
pub static NotifyGrab:i32 = 1;
pub static NotifyUngrab:i32 = 2;
pub static NotifyWhileGrabbed:i32 = 3;

pub static NotifyHint:i32 = 1; /* for MotionNotify events */

/* Notify detail */

pub static NotifyAncestor:i32 = 0;
pub static NotifyVirtual:i32 = 1;
pub static NotifyInferior:i32 = 2;
pub static NotifyNonlinear:i32 = 3;
pub static NotifyNonlinearVirtual:i32 = 4;
pub static NotifyPointer:i32 = 5;
pub static NotifyPointerRoot:i32 = 6;
pub static NotifyDetailNone:i32 = 7;

/* Visibility notify */

pub static VisibilityUnobscured:i32 = 0;
pub static VisibilityPartiallyObscured:i32 = 1;
pub static VisibilityFullyObscured:i32 = 2;

/* Circulation request */

pub static PlaceOnTop:i32 = 0;
pub static PlaceOnBottom:i32 = 1;

/* protocol families */

pub static FamilyInternet:i32 = 0; /* IPv4 */
pub static FamilyDECnet:i32 = 1;
pub static FamilyChaos:i32 = 2;
pub static FamilyInternet6:i32 = 6; /* IPv6 */

/* authentication families not tied to a specific protocol */
pub static FamilyServerInterpreted:i32 = 5;

/* Property notification */

pub static PropertyNewValue:i32 = 0;
pub static PropertyDelete:i32 = 1;

/* Color Map notification */

pub static ColormapUninstalled:i32 = 0;
pub static ColormapInstalled:i32 = 1;

/* GrabPointer, GrabButton, GrabKeyboard, GrabKey Modes */

pub static GrabModeSync:i32 = 0;
pub static GrabModeAsync:i32 = 1;

/* GrabPointer, GrabKeyboard reply status */

pub static GrabSuccess:i32 = 0;
pub static AlreadyGrabbed:i32 = 1;
pub static GrabInvalidTime:i32 = 2;
pub static GrabNotViewable:i32 = 3;
pub static GrabFrozen:i32 = 4;

/* AllowEvents modes */

pub static AsyncPointer:i32 = 0;
pub static SyncPointer:i32 = 1;
pub static ReplayPointer:i32 = 2;
pub static AsyncKeyboard:i32 = 3;
pub static SyncKeyboard:i32 = 4;
pub static ReplayKeyboard:i32 = 5;
pub static AsyncBoth:i32 = 6;
pub static SyncBoth:i32 = 7;

/* Used in SetInputFocus, GetInputFocus */

pub static RevertToNone:i32 = None as i32;
pub static RevertToPointerRoot:i32 = PointerRoot as i32;
pub static RevertToParent:i32 = 2;

/*****************************************************************
 * ERROR CODES 
 *****************************************************************/

pub static Success:i32 = 0;	/* everything's okay */
pub static BadRequest:i32 = 1;	/* bad request code */
pub static BadValue:i32 = 2;	/* int parameter out of range */
pub static BadWindow:i32 = 3;	/* parameter not a Window */
pub static BadPixmap:i32 = 4;	/* parameter not a Pixmap */
pub static BadAtom:i32 = 5;	/* parameter not an Atom */
pub static BadCursor:i32 = 6;	/* parameter not a Cursor */
pub static BadFont:i32 = 7;	/* parameter not a Font */
pub static BadMatch:i32 = 8;	/* parameter mismatch */
pub static BadDrawable:i32 = 9;	/* parameter not a Pixmap or Window */
pub static BadAccess:i32 = 10;	/* depending on context:
				 - key/button already grabbed
				 - attempt to free an illegal 
				   cmap entry 
				- attempt to store into a read-only 
				   color map entry.
 				- attempt to modify the access control
				   list from other than the local host.
				*/
pub static BadAlloc:i32 = 11;	/* insufficient resources */
pub static BadColor:i32 = 12;	/* no such colormap */
pub static BadGC:i32 = 13;	/* parameter not a GC */
pub static BadIDChoice:i32 = 14;	/* choice not in range or already used */
pub static BadName:i32 = 15;	/* font or color name doesn't exist */
pub static BadLength:i32 = 16;	/* Request length incorrect */
pub static BadImplementation:i32 = 17; /* server is defective */

pub static FirstExtensionError:i32 = 128;
pub static LastExtensionError:i32 = 255;

/*****************************************************************
 * WINDOW DEFINITIONS 
 *****************************************************************/

/* Window classes used by CreateWindow */
/* Note that CopyFromParent is already defined as 0 above */

pub static InputOutput:i32 = 1;
pub static InputOnly:i32 = 2;

/* Window attributes for CreateWindow and ChangeWindowAttributes */

pub static CWBackPixmap:u64 = (1<<0);
pub static CWBackPixel:u64 = (1<<1);
pub static CWBorderPixmap:u64 = (1<<2);
pub static CWBorderPixel:u64 = (1<<3);
pub static CWBitGravity:u64 = (1<<4);
pub static CWWinGravity:u64 = (1<<5);
pub static CWBackingStore:u64 = (1<<6);
pub static CWBackingPlanes:u64 = (1<<7);
pub static CWBackingPixel:u64 = (1<<8);
pub static CWOverrideRedirect:u64 = (1<<9);
pub static CWSaveUnder:u64 = (1<<10);
pub static CWEventMask:u64 = (1<<11);
pub static CWDontPropagate:u64 = (1<<12);
pub static CWColormap:u64 = (1<<13);
pub static CWCursor:u64 = (1<<14);

/* ConfigureWindow structure */

pub static CWX:i32 = (1<<0);
pub static CWY:i32 = (1<<1);
pub static CWWidth:i32 = (1<<2);
pub static CWHeight:i32 = (1<<3);
pub static CWBorderWidth:i32 = (1<<4);
pub static CWSibling:i32 = (1<<5);
pub static CWStackMode:i32 = (1<<6);


/* Bit Gravity */

pub static ForgetGravity:i32 = 0;
pub static NorthWestGravity:i32 = 1;
pub static NorthGravity:i32 = 2;
pub static NorthEastGravity:i32 = 3;
pub static WestGravity:i32 = 4;
pub static CenterGravity:i32 = 5;
pub static EastGravity:i32 = 6;
pub static SouthWestGravity:i32 = 7;
pub static SouthGravity:i32 = 8;
pub static SouthEastGravity:i32 = 9;
pub static StaticGravity:i32 = 10;

/* Window gravity + bit gravity above */

pub static UnmapGravity:i32 = 0;

/* Used in CreateWindow for backing-store hint */

pub static NotUseful:i32 = 0;
pub static WhenMapped:i32 = 1;
pub static Always:i32 = 2;

/* Used in GetWindowAttributes reply */

pub static IsUnmapped:i32 = 0;
pub static IsUnviewable:i32 = 1;
pub static IsViewable:i32 = 2;

/* Used in ChangeSaveSet */

pub static SetModeInsert:i32 = 0;
pub static SetModeDelete:i32 = 1;

/* Used in ChangeCloseDownMode */

pub static DestroyAll:i32 = 0;
pub static RetainPermanent:i32 = 1;
pub static RetainTemporary:i32 = 2;

/* Window stacking method (in configureWindow) */

pub static Above:i32 = 0;
pub static Below:i32 = 1;
pub static TopIf:i32 = 2;
pub static BottomIf:i32 = 3;
pub static Opposite:i32 = 4;

/* Circulation direction */

pub static RaiseLowest:i32 = 0;
pub static LowerHighest:i32 = 1;

/* Property modes */

pub static PropModeReplace:i32 = 0;
pub static PropModePrepend:i32 = 1;
pub static PropModeAppend:i32 = 2;

/*****************************************************************
 * GRAPHICS DEFINITIONS
 *****************************************************************/

/* graphics functions, as in GC.alu */

pub static GXclear:i32 = 0x0;	/* 0 */
pub static GXand:i32 = 0x1;	/* src AND dst */
pub static GXandReverse:i32 = 0x2;	/* src AND NOT dst */
pub static GXcopy:i32 = 0x3;	/* src */
pub static GXandInverted:i32 = 0x4;	/* NOT src AND dst */
pub static GXnoop:i32 = 0x5;	/* dst */
pub static GXxor:i32 = 0x6;	/* src XOR dst */
pub static GXor:i32 = 0x7;	/* src OR dst */
pub static GXnor:i32 = 0x8;	/* NOT src AND NOT dst */
pub static GXequiv:i32 = 0x9;	/* NOT src XOR dst */
pub static GXinvert:i32 = 0xa;	/* NOT dst */
pub static GXorReverse:i32 = 0xb;	/* src OR NOT dst */
pub static GXcopyInverted:i32 = 0xc;	/* NOT src */
pub static GXorInverted:i32 = 0xd;	/* NOT src OR dst */
pub static GXnand:i32 = 0xe;	/* NOT src OR NOT dst */
pub static GXset:i32 = 0xf;	/* 1 */

/* LineStyle */

pub static LineSolid:i32 = 0;
pub static LineOnOffDash:i32 = 1;
pub static LineDoubleDash:i32 = 2;

/* capStyle */

pub static CapNotLast:i32 = 0;
pub static CapButt:i32 = 1;
pub static CapRound:i32 = 2;
pub static CapProjecting:i32 = 3;

/* joinStyle */

pub static JoinMiter:i32 = 0;
pub static JoinRound:i32 = 1;
pub static JoinBevel:i32 = 2;

/* fillStyle */

pub static FillSolid:i32 = 0;
pub static FillTiled:i32 = 1;
pub static FillStippled:i32 = 2;
pub static FillOpaqueStippled:i32 = 3;

/* fillRule */

pub static EvenOddRule:i32 = 0;
pub static WindingRule:i32 = 1;

/* subwindow mode */

pub static ClipByChildren:i32 = 0;
pub static IncludeInferiors:i32 = 1;

/* SetClipRectangles ordering */

pub static Unsorted:i32 = 0;
pub static YSorted:i32 = 1;
pub static YXSorted:i32 = 2;
pub static YXBanded:i32 = 3;

/* CoordinateMode for drawing routines */

pub static CoordModeOrigin:i32 = 0; /* relative to the origin */
pub static CoordModePrevious:i32 = 1; /* relative to previous point */

/* Polygon shapes */

pub static Complex:i32 = 0;	/* paths may intersect */
pub static Nonconvex:i32 = 1;	/* no paths intersect, but not convex */
pub static Convex:i32 = 2;	/* wholly convex */

/* Arc modes for PolyFillArc */

pub static ArcChord:i32 = 0;	/* join endpoints of arc */
pub static ArcPieSlice:i32 = 1;	/* join endpoints to center of arc */

/* GC components: masks used in CreateGC, CopyGC, ChangeGC, OR'ed into
   GC.stateChanges */

pub static GCFunction:i64 = (1<<0);
pub static GCPlaneMask:i64 = (1<<1);
pub static GCForeground:i64 = (1<<2);
pub static GCBackground:i64 = (1<<3);
pub static GCLineWidth:i64 = (1<<4);
pub static GCLineStyle:i64 = (1<<5);
pub static GCCapStyle:i64 = (1<<6);
pub static GCJoinStyle:i64 = (1<<7);
pub static GCFillStyle:i64 = (1<<8);
pub static GCFillRule:i64 = (1<<9);
pub static GCTile:i64 = (1<<10);
pub static GCStipple:i64 = (1<<11);
pub static GCTileStipXOrigin:i64 = (1<<12);
pub static GCTileStipYOrigin:i64 = (1<<13);
pub static GCFont:i64 = (1<<14);
pub static GCSubwindowMode:i64 = (1<<15);
pub static GCGraphicsExposures:i64 = (1<<16);
pub static GCClipXOrigin:i64 = (1<<17);
pub static GCClipYOrigin:i64 = (1<<18);
pub static GCClipMask:i64 = (1<<19);
pub static GCDashOffset:i64 = (1<<20);
pub static GCDashList:i64 = (1<<21);
pub static GCArcMode:i64 = (1<<22);

pub static GCLastBit:i64 = 22;
/*****************************************************************
 * FONTS 
 *****************************************************************/

/* used in QueryFont -- draw direction */

pub static FontLeftToRight:i32 = 0;
pub static FontRightToLeft:i32 = 1;

pub static FontChange:i32 = 255;

/*****************************************************************
 *  IMAGING 
 *****************************************************************/

/* ImageFormat -- PutImage, GetImage */

pub static XYBitmap:i32 = 0;	/* depth 1, XYFormat */
pub static XYPixmap:i32 = 1;	/* depth == drawable depth */
pub static ZPixmap:i32 = 2;	/* depth == drawable depth */

/*****************************************************************
 *  COLOR MAP STUFF 
 *****************************************************************/

/* For CreateColormap */

pub static AllocNone:i32 = 0;	/* create map with no entries */
pub static AllocAll:i32 = 1;	/* allocate entire map writeable */


/* Flags used in StoreNamedColor, StoreColors */

pub static DoRed:i32 = (1<<0);
pub static DoGreen:i32 = (1<<1);
pub static DoBlue:i32 = (1<<2);

/*****************************************************************
 * CURSOR STUFF
 *****************************************************************/

/* QueryBestSize Class */

pub static CursorShape:i32 = 0;	/* largest size that can be displayed */
pub static TileShape:i32 = 1;	/* size tiled fastest */
pub static StippleShape:i32 = 2;	/* size stippled fastest */

/***************************************************************** 
 * KEYBOARD/POINTER STUFF
 *****************************************************************/

pub static AutoRepeatModeOff:i32 = 0;
pub static AutoRepeatModeOn:i32 = 1;
pub static AutoRepeatModeDefault:i32 = 2;

pub static LedModeOff:i32 = 0;
pub static LedModeOn:i32 = 1;

/* masks for ChangeKeyboardControl */

pub static KBKeyClickPercent:i64 = (1<<0);
pub static KBBellPercent:i64 = (1<<1);
pub static KBBellPitch:i64 = (1<<2);
pub static KBBellDuration:i64 = (1<<3);
pub static KBLed:i64 = (1<<4);
pub static KBLedMode:i64 = (1<<5);
pub static KBKey:i64 = (1<<6);
pub static KBAutoRepeatMode:i64 = (1<<7);

pub static MappingSuccess:i32 = 0;
pub static MappingBusy:i32 = 1;
pub static MappingFailed:i32 = 2;

pub static MappingModifier:i32 = 0;
pub static MappingKeyboard:i32 = 1;
pub static MappingPointer:i32 = 2;

/*****************************************************************
 * SCREEN SAVER STUFF 
 *****************************************************************/

pub static DontPreferBlanking:i32 = 0;
pub static PreferBlanking:i32 = 1;
pub static DefaultBlanking:i32 = 2;

pub static DisableScreenSaver:i32 = 0;
pub static DisableScreenInterval:i32 = 0;

pub static DontAllowExposures:i32 = 0;
pub static AllowExposures:i32 = 1;
pub static DefaultExposures:i32 = 2;

/* for ForceScreenSaver */

pub static ScreenSaverReset:i32 = 0;
pub static ScreenSaverActive:i32 = 1;

/*****************************************************************
 * HOSTS AND CONNECTIONS
 *****************************************************************/

/* for ChangeHosts */

pub static HostInsert:i32 = 0;
pub static HostDelete:i32 = 1;

/* for ChangeAccessControl */

pub static EnableAccess:i32 = 1;
pub static DisableAccess:i32 = 0;

/* Display classes  used in opening the connection 
 * Note that the statically allocated ones are even numbered and the
 * dynamically changeable ones are odd numbered */

pub static StaticGray:i32 = 0;
pub static GrayScale:i32 = 1;
pub static StaticColor:i32 = 2;
pub static PseudoColor:i32 = 3;
pub static TrueColor:i32 = 4;
pub static DirectColor:i32 = 5;


/* Byte order  used in imageByteOrder and bitmapBitOrder */

pub static LSBFirst:i32 = 0;
pub static MSBFirst:i32 = 1;

pub mod cursorfont {
    pub static XC_num_glyphs:u32 = 154;
    pub static XC_X_cursor:u32 = 0;
    pub static XC_arrow:u32 = 2;
    pub static XC_based_arrow_down:u32 = 4;
    pub static XC_based_arrow_up:u32 = 6;
    pub static XC_boat:u32 = 8;
    pub static XC_bogosity:u32 = 10;
    pub static XC_bottom_left_corner:u32 = 12;
    pub static XC_bottom_right_corner:u32 = 14;
    pub static XC_bottom_side:u32 = 16;
    pub static XC_bottom_tee:u32 = 18;
    pub static XC_box_spiral:u32 = 20;
    pub static XC_center_ptr:u32 = 22;
    pub static XC_circle:u32 = 24;
    pub static XC_clock:u32 = 26;
    pub static XC_coffee_mug:u32 = 28;
    pub static XC_cross:u32 = 30;
    pub static XC_cross_reverse:u32 = 32;
    pub static XC_crosshair:u32 = 34;
    pub static XC_diamond_cross:u32 = 36;
    pub static XC_dot:u32 = 38;
    pub static XC_dotbox:u32 = 40;
    pub static XC_double_arrow:u32 = 42;
    pub static XC_draft_large:u32 = 44;
    pub static XC_draft_small:u32 = 46;
    pub static XC_draped_box:u32 = 48;
    pub static XC_exchange:u32 = 50;
    pub static XC_fleur:u32 = 52;
    pub static XC_gobbler:u32 = 54;
    pub static XC_gumby:u32 = 56;
    pub static XC_hand1:u32 = 58;
    pub static XC_hand2:u32 = 60;
    pub static XC_heart:u32 = 62;
    pub static XC_icon:u32 = 64;
    pub static XC_iron_cross:u32 = 66;
    pub static XC_left_ptr:u32 = 68;
    pub static XC_left_side:u32 = 70;
    pub static XC_left_tee:u32 = 72;
    pub static XC_leftbutton:u32 = 74;
    pub static XC_ll_angle:u32 = 76;
    pub static XC_lr_angle:u32 = 78;
    pub static XC_man:u32 = 80;
    pub static XC_middlebutton:u32 = 82;
    pub static XC_mouse:u32 = 84;
    pub static XC_pencil:u32 = 86;
    pub static XC_pirate:u32 = 88;
    pub static XC_plus:u32 = 90;
    pub static XC_question_arrow:u32 = 92;
    pub static XC_right_ptr:u32 = 94;
    pub static XC_right_side:u32 = 96;
    pub static XC_right_tee:u32 = 98;
    pub static XC_rightbutton:u32 = 100;
    pub static XC_rtl_logo:u32 = 102;
    pub static XC_sailboat:u32 = 104;
    pub static XC_sb_down_arrow:u32 = 106;
    pub static XC_sb_h_double_arrow:u32 = 108;
    pub static XC_sb_left_arrow:u32 = 110;
    pub static XC_sb_right_arrow:u32 = 112;
    pub static XC_sb_up_arrow:u32 = 114;
    pub static XC_sb_v_double_arrow:u32 = 116;
    pub static XC_shuttle:u32 = 118;
    pub static XC_sizing:u32 = 120;
    pub static XC_spider:u32 = 122;
    pub static XC_spraycan:u32 = 124;
    pub static XC_star:u32 = 126;
    pub static XC_target:u32 = 128;
    pub static XC_tcross:u32 = 130;
    pub static XC_top_left_arrow:u32 = 132;
    pub static XC_top_left_corner:u32 = 134;
    pub static XC_top_right_corner:u32 = 136;
    pub static XC_top_side:u32 = 138;
    pub static XC_top_tee:u32 = 140;
    pub static XC_trek:u32 = 142;
    pub static XC_ul_angle:u32 = 144;
    pub static XC_umbrella:u32 = 146;
    pub static XC_ur_angle:u32 = 148;
    pub static XC_watch:u32 = 150;
    pub static XC_xterm:u32 = 152;
}
