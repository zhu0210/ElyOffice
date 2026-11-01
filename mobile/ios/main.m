// ElyMark on iOS: UIKit starts, then GPUI draws the window.

#import <UIKit/UIKit.h>
#include <stdbool.h>

void ely_ios_main(void);
void *gpui_ios_get_window(void);
bool gpui_ios_request_frame(void *window);
void gpui_ios_set_frame_waker(void *window, void (*waker)(void *context), void *context);
void gpui_ios_will_enter_foreground(void *app);
void gpui_ios_did_become_active(void *app);
void gpui_ios_will_resign_active(void *app);
void gpui_ios_did_enter_background(void *app);
void gpui_ios_will_terminate(void *app);

@interface ElyAppDelegate : UIResponder <UIApplicationDelegate>
@property (nonatomic, strong) UIWindow *window;
@property (nonatomic, assign) void *gpuiWindow;
@property (nonatomic, strong) CADisplayLink *displayLink;
@end

static void ElyResumeFrames(void *context);

@implementation ElyAppDelegate

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)options {
    ely_ios_main();
    self.gpuiWindow = gpui_ios_get_window();
    if (!self.gpuiWindow) {
        NSLog(@"ElyMark: GPUI opened no window");
        return YES;
    }
    [self startFrames];
    return YES;
}

// Paused between frames; GPUI's waker resumes it.
- (void)startFrames {
    self.displayLink = [CADisplayLink displayLinkWithTarget:self selector:@selector(renderFrame)];
    [self.displayLink addToRunLoop:[NSRunLoop mainRunLoop] forMode:NSRunLoopCommonModes];
    gpui_ios_set_frame_waker(self.gpuiWindow, ElyResumeFrames, (__bridge void *)self);
}

- (void)renderFrame {
    if (!gpui_ios_request_frame(self.gpuiWindow)) {
        self.displayLink.paused = YES;
    }
}

static void ElyResumeFrames(void *context) {
    ((__bridge ElyAppDelegate *)context).displayLink.paused = NO;
}

- (void)applicationWillEnterForeground:(UIApplication *)application {
    gpui_ios_will_enter_foreground(NULL);
    if (!self.displayLink && self.gpuiWindow) {
        [self startFrames];
    }
}

- (void)applicationDidBecomeActive:(UIApplication *)application {
    gpui_ios_did_become_active(NULL);
}

- (void)applicationWillResignActive:(UIApplication *)application {
    gpui_ios_will_resign_active(NULL);
}

- (void)applicationDidEnterBackground:(UIApplication *)application {
    gpui_ios_did_enter_background(NULL);
    [self.displayLink invalidate];
    self.displayLink = nil;
}

- (void)applicationWillTerminate:(UIApplication *)application {
    [self.displayLink invalidate];
    self.displayLink = nil;
    gpui_ios_will_terminate(NULL);
}

@end

int main(int argc, char *argv[]) {
    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass([ElyAppDelegate class]));
    }
}
