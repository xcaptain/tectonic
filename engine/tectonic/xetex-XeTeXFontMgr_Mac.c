/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
 Copyright (c) 2012, 2013 by Jiang Jiang

 SIL Author(s): Jonathan Kew

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the copyright holders
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in this Software without prior written
authorization from the copyright holders.
\****************************************************************************/
#ifdef XETEX_MAC

#include "xetex-core.h"
#include "xetex-XeTeXFontMgr_Mac.h"

#if 0
#include <Cocoa/Cocoa.h>  //UNUSABLE in Pure C
#endif

void* raw_objc(const char*);
struct NSArray;
typedef struct NSArray NSArray;
struct NSString;
typedef struct NSString NSString;
struct NSAutoreleasePool;
typedef struct NSAutoreleasePool NSAutoreleasePool;
struct NSEnumerator;
typedef struct NSEnumerator NSEnumerator;
struct NSFont;
typedef struct NSFont NSFont;

typedef size_t id;
const char* NSString_cstr(const NSString* self);

CTFontDescriptorRef XeTeXFontMgr_findFontWithName(CFStringRef name, CFStringRef key)
{
    CFStringRef keys[] = { key };
    CFTypeRef values[] = { name };
    CFDictionaryRef attributes = CFDictionaryCreate(NULL, (const void **) &keys, (const void **) &values, 1,
        &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);
    CTFontDescriptorRef descriptor = CTFontDescriptorCreateWithAttributes(attributes);
    CFRelease(attributes);

    CFSetRef mandatoryAttributes = CFSetCreate(NULL, (const void **) &keys, 1, &kCFTypeSetCallBacks);
    CFArrayRef matches = CTFontDescriptorCreateMatchingFontDescriptors(descriptor, mandatoryAttributes);
    CFRelease(mandatoryAttributes);
    CFRelease(descriptor);

    CTFontDescriptorRef matched = NULL;
    if (matches) {
        if (CFArrayGetCount(matches)) {
            matched = (CTFontDescriptorRef) CFArrayGetValueAtIndex(matches, 0);
            CFRetain(matched);
        }
        CFRelease(matches);
    }
    return matched;
}

void
XeTeXFontMgr_Mac_appendNameToList(XeTeXFontMgr* self, CTFontRef font,
                                   CppStdListOfString* nameList,
                                   CFStringRef nameKey)
{
    CFStringRef name = CTFontCopyName(font, nameKey);
    if (name) {
        XeTeXFontMgr_appendToList(self, nameList, NSString_cstr((NSString*)raw_objc("[(NSString *) name UTF8String]")));
        CFRelease(name);
    }
    CFStringRef language;
    name = CTFontCopyLocalizedName(font, nameKey, &language);
    if (name) {
        XeTeXFontMgr_appendToList(self, nameList, NSString_cstr((NSString*)raw_objc("[(NSString *) name UTF8String]")));
        CFRelease(name);
    }
}

XeTeXFontMgrNameCollection*
XeTeXFontMgr_Mac_readNames(XeTeXFontMgr* self, CTFontDescriptorRef fontRef)
{
    XeTeXFontMgrNameCollection* names = XeTeXFontMgrNameCollection_create();

    CFStringRef psName = (CFStringRef) CTFontDescriptorCopyAttribute(fontRef, kCTFontNameAttribute);
    if (!psName)
        return names;

    NSAutoreleasePool *pool = (NSAutoreleasePool*)raw_objc("[NSAutoreleasePool new]");

    CppStdString_assign_from_const_char_ptr(names->m_psName, NSString_cstr((NSString*)raw_objc("[(NSString *) psName UTF8String]")));
    CFRelease(psName);

    CTFontRef font = CTFontCreateWithFontDescriptor(fontRef, 0.0, 0);
    XeTeXFontMgr_Mac_appendNameToList(self, font, names->m_fullNames,   kCTFontFullNameKey);
    XeTeXFontMgr_Mac_appendNameToList(self, font, names->m_familyNames, kCTFontFamilyNameKey);
    XeTeXFontMgr_Mac_appendNameToList(self, font, names->m_styleNames,  kCTFontStyleNameKey);
    CFRelease(font);

    raw_objc("[pool release]");

    return names;
}

void
XeTeXFontMgr_Mac_addFontsToCaches(XeTeXFontMgr* self, CFArrayRef fonts)
{
    NSEnumerator* enumerator = (NSEnumerator*)raw_objc("[(NSArray*)fonts objectEnumerator]");
    while (true) {
        id aFont = (id)raw_objc("[enumerator nextObject]");
        if(!aFont)
	    break;
        CTFontDescriptorRef fontRef = XeTeXFontMgr_findFontWithName((CFStringRef)raw_objc("[aFont objectAtIndex: 0]"), kCTFontNameAttribute);
        XeTeXFontMgrNameCollection* names = XeTeXFontMgr_readNames(self, fontRef);
        XeTeXFontMgr_addToMaps(self, fontRef, names);
        XeTeXFontMgrNameCollection_delete(names);
    }
}

void
XeTeXFontMgr_Mac_addFamilyToCaches(XeTeXFontMgr* self, CTFontDescriptorRef familyRef)
{
    CFStringRef nameStr = (CFStringRef) CTFontDescriptorCopyAttribute(familyRef, kCTFontFamilyNameAttribute);
    if (nameStr) {
        NSArray* members = (NSArray*)raw_objc("[[NSFontManager sharedFontManager]"
                            "availableMembersOfFontFamily: (NSString*)nameStr]");
        CFRelease(nameStr);
        XeTeXFontMgr_Mac_addFontsToCaches(self, (CFArrayRef)members);
    }
}

void
XeTeXFontMgr_Mac_addFontAndSiblingsToCaches(XeTeXFontMgr* self, CTFontDescriptorRef fontRef)
{
    CFStringRef name = (CFStringRef) CTFontDescriptorCopyAttribute(fontRef, kCTFontNameAttribute);
    if (name) {
        NSFont* font = (NSFont*)raw_objc("[NSFont fontWithName:(NSString*)name size:10.0]");
        CFRelease(name);
        NSArray* members = (NSArray*)raw_objc("[[NSFontManager sharedFontManager]"
                            "availableMembersOfFontFamily: [font familyName]]");
        XeTeXFontMgr_Mac_addFontsToCaches(self, (CFArrayRef)members);
    }
}

void
XeTeXFontMgr_Mac_searchForHostPlatformFonts(XeTeXFontMgr* self, const char* name)
{
    // the name might be:
    //  FullName
    //  Family-Style (if there's a hyphen)
    //  PSName
    //  Family
    // ...so we need to try it as each of these

    CFStringRef nameStr = CFStringCreateWithCString(kCFAllocatorDefault, name, kCFStringEncodingUTF8);
    CTFontDescriptorRef matched = XeTeXFontMgr_findFontWithName(nameStr, kCTFontDisplayNameAttribute);
    if (matched) {
        // found it, so locate the family, and add all members to the caches
        XeTeXFontMgr_Mac_addFontAndSiblingsToCaches(self, matched);
        CFRelease(matched);
        return;
    }

    const char* hyph_pos = strchr(name, '-');
    int hyph = hyph_pos ? hyph_pos - name : -1;
    if (hyph > 0 && hyph < strlen(name) - 1) {
        CppStdString* family = CppStdString_create();
        CppStdString_assign_n_chars(family, name, hyph);
        CFStringRef familyStr = CFStringCreateWithCString(kCFAllocatorDefault, CppStdString_cstr(family), kCFStringEncodingUTF8);
        CppStdString_delete(family);

        NSArray* familyMembers = (NSArray*)raw_objc("[[NSFontManager sharedFontManager]"
                                  "availableMembersOfFontFamily: (NSString*)familyStr]");
        if ((int)raw_objc("[familyMembers count]") > 0) {
            XeTeXFontMgr_Mac_addFontsToCaches(self, (CFArrayRef)familyMembers);
            return;
        }

        matched = XeTeXFontMgr_findFontWithName(familyStr, kCTFontFamilyNameAttribute);
        if (matched) {
            XeTeXFontMgr_Mac_addFamilyToCaches(self, matched);
            CFRelease(matched);
            return;
        }
    }

    matched = XeTeXFontMgr_findFontWithName(nameStr, kCTFontNameAttribute);
    if (matched) {
        XeTeXFontMgr_Mac_addFontAndSiblingsToCaches(self, matched);
        CFRelease(matched);
        return;
    }

    NSArray* familyMembers = (NSArray*)raw_objc("[[NSFontManager sharedFontManager]"
                              "availableMembersOfFontFamily: (NSString*)nameStr]");
    if ((int)raw_objc("[familyMembers count]") > 0) {
        XeTeXFontMgr_Mac_addFontsToCaches(self, (CFArrayRef)familyMembers);
        return;
    }

    matched = XeTeXFontMgr_findFontWithName(nameStr, kCTFontFamilyNameAttribute);
    if (matched) {
        XeTeXFontMgr_Mac_addFamilyToCaches(self, matched);
        CFRelease(matched);
        return;
    }
}

NSAutoreleasePool* pool = NULL;

void
XeTeXFontMgr_Mac_initialize(XeTeXFontMgr* self)
{
    pool = (NSAutoreleasePool*)raw_objc("[[NSAutoreleasePool alloc] init]");
}

void
XeTeXFontMgr_Mac_terminate(XeTeXFontMgr* self)
{
    if (pool != NULL) {
        raw_objc("[pool release]");
    }
}

char*
XeTeXFontMgr_Mac_getPlatformFontDesc(const XeTeXFontMgr* self, PlatformFontRef descriptor)
{
    char* path = NULL;
    CTFontRef ctFont = CTFontCreateWithFontDescriptor(descriptor, 0.0, 0);
    if (ctFont) {
        CFURLRef url = NULL;
#if !defined(MAC_OS_X_VERSION_10_6) || MAC_OS_X_VERSION_MIN_REQUIRED < MAC_OS_X_VERSION_10_6
        /* kCTFontURLAttribute was not avialable before 10.6 */
        FSRef fsref;
        ATSFontRef atsFont = CTFontGetPlatformFont(ctFont, NULL);
        OSStatus status = ATSFontGetFileReference(atsFont, &fsref);
        if (status == noErr)
            url = CFURLCreateFromFSRef(NULL, &fsref);
#else
        url = (CFURLRef) CTFontCopyAttribute(ctFont, kCTFontURLAttribute);
#endif
        if (url) {
            UInt8 posixPath[PATH_MAX];
            if (CFURLGetFileSystemRepresentation(url, true, posixPath, PATH_MAX)) {
                path = strdup((char*)posixPath);
            }
            CFRelease(url);
        }
        CFRelease(ctFont);
    }
    if (strlen(path) == 0) {
        free(path);
        path = NULL;
    }
    if (!path)
        path = strdup("[unknown]");
    return strdup(path);
}

void XeTeXFontMgr_Mac_ctor(XeTeXFontMgr_Mac* self)
{
    XeTeXFontMgr_base_ctor(&self->super_);
    self->super_.m_memfnInitialize = XeTeXFontMgr_Mac_initialize;
    self->super_.m_memfnTerminate = XeTeXFontMgr_Mac_terminate;
    self->super_.m_memfnGetPlatformFontDesc = XeTeXFontMgr_Mac_getPlatformFontDesc;
    self->super_.m_memfnSearchForHostPlatformFonts = XeTeXFontMgr_Mac_searchForHostPlatformFonts;
    self->super_.m_memfnReadNames = XeTeXFontMgr_Mac_readNames; 
}

XeTeXFontMgr_Mac* XeTeXFontMgr_Mac_create() {
	XeTeXFontMgr_Mac* self = (XeTeXFontMgr_Mac*)malloc(sizeof(XeTeXFontMgr_Mac));
        XeTeXFontMgr_Mac_ctor(self);
	return self;
}

#endif

