#include <CoreFoundation/CoreFoundation.h>
#include <CoreServices/CoreServices.h>
#include <QuickLook/QuickLook.h>

#include <qlhighlight.h>

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
        
        NSURL *nsurl = (__bridge NSURL *)url;
        NSData *data = [NSData dataWithContentsOfURL:nsurl];
        if (!data) { return noErr; }
        
        // The above might have taken some time, so before proceeding make sure the user didn't cancel the request
        if (QLPreviewRequestIsCancelled(preview)) { return noErr; }

        
        NSString *html = @(highlight_html());
        
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
