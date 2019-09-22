/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
 Copyright (c) 2012, 2013 by Jiang Jiang
 Copyright (c) 2012-2015 by Khaled Hosny

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

/*
 *   file name:  XeTeXFontInst_Mac.cpp
 *
 *   created on: 2005-10-22
 *   created by: Jonathan Kew
 */

#ifdef XETEX_MAC

#include "xetex-core.h"
#include "xetex-XeTeXFontInst_Mac.h"
#include "xetex-ext.h"

void XeTeXFontInst_Mac_dtor(XeTeXFontInst* self)
{
    XeTeXFontInst_Mac* real_self = (XeTeXFontInst_Mac*)self;
    if (real_self->m_descriptor != 0)
        CFRelease(real_self->m_descriptor);
    if (real_self->m_fontRef != 0)
        CFRelease(real_self->m_fontRef);
}


void
XeTeXFontInst_Mac_initialize(XeTeXFontInst_Mac* self, int *status)
{
    if (self->m_descriptor == 0) {
        *status = 1;
        return;
    }

    if (*status != 0)
        self->m_descriptor = 0;

    // Create a copy of original font descriptor with font cascading (fallback) disabled
    CFArrayRef emptyCascadeList = CFArrayCreate(NULL, NULL, 0, &kCFTypeArrayCallBacks);
    const void* values[] = { emptyCascadeList };
    const void* attributeKeys[] = { kCTFontCascadeListAttribute };
    CFDictionaryRef attributes = CFDictionaryCreate(NULL, attributeKeys, values, 1,
        &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);
    CFRelease(emptyCascadeList);

    self->m_descriptor = CTFontDescriptorCreateCopyWithAttributes(self->m_descriptor, attributes);
    CFRelease(attributes);
    self->m_fontRef = CTFontCreateWithFontDescriptor(self->m_descriptor, self->super_.m_pointSize * 72.0 / 72.27, NULL);
    if (self->m_fontRef) {
        char *pathname;
        uint32_t index;
        pathname = getFileNameFromCTFont(self->m_fontRef, &index);

        XeTeXFontInst_initialize(&self->super_, pathname, index, status);
    } else {
        *status = 1;
        CFRelease(self->m_descriptor);
        self->m_descriptor = 0;
    }
}

void XeTeXFontInst_Mac_ctor(XeTeXFontInst_Mac* self, CTFontDescriptorRef descriptor, float pointSize, int *status) {
	XeTeXFontInst_base_ctor(&self->super_, NULL, 0, pointSize, status);
	self->super_.m_subdtor = XeTeXFontInst_Mac_dtor;
	self->m_descriptor = descriptor;
	self->m_fontRef = 0;
	XeTeXFontInst_Mac_initialize(self, status);
}

XeTeXFontInst_Mac* XeTeXFontInst_Mac_create(CTFontDescriptorRef descriptor, float pointSize, int *status) {
	XeTeXFontInst_Mac* value = malloc(sizeof(XeTeXFontInst_Mac));
	XeTeXFontInst_Mac_ctor(value, descriptor, pointSize, status);
	return value;
}



#endif

