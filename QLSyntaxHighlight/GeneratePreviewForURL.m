#include <CoreFoundation/CoreFoundation.h>
#include <CoreServices/CoreServices.h>
#include <QuickLook/QuickLook.h>

#import <Foundation/Foundation.h>

OSStatus GeneratePreviewForURL(void *thisInterface, QLPreviewRequestRef preview, CFURLRef url, CFStringRef contentTypeUTI, CFDictionaryRef options);
void CancelPreviewGeneration(void *thisInterface, QLPreviewRequestRef preview);

/* -----------------------------------------------------------------------------
 Generate a preview for file
 
 This function's job is to create preview for designated file
 ----------------------------------------------------------------------------- */

OSStatus GeneratePreviewForURL(void *thisInterface,
                               QLPreviewRequestRef preview,
                               CFURLRef url,
                               CFStringRef contentTypeUTI,
                               CFDictionaryRef options) {
    @autoreleasepool {
        
        // Load the property list from the URL
        NSURL *nsurl = (__bridge NSURL *)url;
        NSData *data = [NSData dataWithContentsOfURL:nsurl];
        if (!data) { return noErr; }
        
        // The above might have taken some time, so before proceeding make sure the user didn't cancel the request
        if (QLPreviewRequestIsCancelled(preview)) { return noErr; }
        
        
        NSString *html = @"<head><title>/Users/me/Desktop/hello/src/main.rs</title><style>\n pre {\n font-size:13px;\n font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;\n }</style></head>\n<body style=\"background-color:#2b303b;\">\n<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {</span>\n<span style=\"color:#c0c5ce;\">    println!(&quot;</span><span style=\"color:#a3be8c;\">Hello, world!</span><span style=\"color:#c0c5ce;\">&quot;);</span>\n<span style=\"color:#c0c5ce;\">}</span>\n</pre>\n</body>";
        
        // Put metadata and attachment in a dictionary
        NSDictionary *properties = @{ // properties for the HTML data
            (__bridge NSString *)kQLPreviewPropertyTextEncodingNameKey : @"UTF-8",
            (__bridge NSString *)kQLPreviewPropertyMIMETypeKey : @"text/html",
        };
        
        // Pass preview data and metadata/attachment dictionary to QuickLook
        QLPreviewRequestSetDataRepresentation(preview,
                                              (__bridge CFDataRef)[html dataUsingEncoding:NSUTF8StringEncoding],
                                              kUTTypeHTML,
                                              (__bridge CFDictionaryRef)properties);
    }
    return noErr;
}

void CancelPreviewGeneration(void *thisInterface, QLPreviewRequestRef preview)
{
    // Implement only if supported
}
