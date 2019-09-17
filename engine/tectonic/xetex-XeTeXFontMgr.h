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

#ifndef __XETEX_FONT_MANAGER_H
#define __XETEX_FONT_MANAGER_H

#include "xetex-core.h"

struct XeTeXFontMgrOpSizeRec {
	unsigned int    designSize;
	unsigned int    subFamilyID;
	unsigned int    nameCode;
	unsigned int    minSize;
	unsigned int    maxSize;
};
typedef struct XeTeXFontMgrOpSizeRec XeTeXFontMgrOpSizeRec;

struct CppStdString;
typedef struct CppStdString CppStdString;

CppStdString* CppStdString_create();
void CppStdString_delete(CppStdString* self);

struct CppStdMapStr2Font;
typedef struct CppStdMapStr2Font CppStdMapStr2Font;

CppStdMapStr2Font* CppStdMapStr2Font_create();
void CppStdMapStr2Font_delete(CppStdMapStr2Font* self);

struct XeTeXFontMgrFamily {
	CppStdMapStr2Font*    styles;
	uint16_t              minWeight;
	uint16_t              maxWeight;
	uint16_t              minWidth;
	uint16_t              maxWidth;
	int16_t               minSlant;
	int16_t               maxSlant;
};

typedef struct XeTeXFontMgrFamily XeTeXFontMgrFamily;

inline XeTeXFontMgrFamily* XeTeXFontMgrFamily_create()
{
	XeTeXFontMgrFamily* self = malloc(sizeof(XeTeXFontMgrFamily));
	self->minWeight = (0);
	self->maxWeight = (0);
	self->minWidth = (0);
	self->maxWidth = (0);
	self->minSlant = (0);
	self->maxSlant = (0);
    self->styles = CppStdMapStr2Font_create();
}
inline void XeTeXFontMgrFamily_delete(XeTeXFontMgrFamily* self)
{
	if(!self)
		return;
	CppStdMapStr2Font_delete(self->styles);
	free(self);
}

struct XeTeXFontMgrFont {
	CppStdString*    m_fullName;
	CppStdString*    m_psName;
	CppStdString*    m_familyName; // default family and style names that should locate this font
	CppStdString*    m_styleName;
	XeTeXFontMgrFamily*         parent;
	PlatformFontRef fontRef;
	XeTeXFontMgrOpSizeRec       opSizeInfo;
	uint16_t        weight;
	uint16_t        width;
	int16_t         slant;
	bool            isReg;
	bool            isBold;
	bool            isItalic;
};
typedef struct XeTeXFontMgrFont XeTeXFontMgrFont;

inline XeTeXFontMgrFont* XeTeXFontMgrFont_create(PlatformFontRef ref) {
	XeTeXFontMgrFont* self = malloc(sizeof(XeTeXFontMgrFont));
	self->m_fullName = (NULL);
	self->m_psName = (NULL);
	self->m_familyName = (NULL);
	self->m_styleName = (NULL);
	self->parent = (NULL);
	self->fontRef = (ref);
	self->weight = (0);
	self->width = (0);
	self->slant = (0);
	self->isReg = (false);
	self->isBold = (false);
	self->isItalic = (false);
	self->opSizeInfo.subFamilyID = 0;
	self->opSizeInfo.designSize = 100; /* default to 10bp */
	return self;
}

inline void XeTeXFontMgrFont_delete(XeTeXFontMgrFont* self) {
	if(!self)
		return;
	CppStdString_delete(self->m_fullName);
	CppStdString_delete(self->m_psName);
	free(self);
}

struct CppStdListOfString;
typedef struct CppStdListOfString CppStdListOfString;

CppStdListOfString* CppStdListOfString_create();
void CppStdListOfString_delete(CppStdListOfString* self);

struct XeTeXFontMgrNameCollection {
	CppStdListOfString*  m_familyNames;
	CppStdListOfString*  m_styleNames;
	CppStdListOfString*  m_fullNames;
	CppStdString*        m_psName;
	CppStdString*        m_subFamily;
};
typedef struct XeTeXFontMgrNameCollection XeTeXFontMgrNameCollection;

inline XeTeXFontMgrNameCollection* XeTeXFontMgrNameCollection_create() {
	XeTeXFontMgrNameCollection* self = malloc(sizeof(XeTeXFontMgrNameCollection));
	self->m_familyNames = CppStdListOfString_create();
	self->m_styleNames = CppStdListOfString_create();
	self->m_fullNames = CppStdListOfString_create();
	self->m_psName = CppStdString_create();
	self->m_subFamily = CppStdString_create();
	return self;
}

inline void XeTeXFontMgrNameCollection_delete(XeTeXFontMgrNameCollection* self) {
	if(!self)
		return;
	CppStdListOfString_delete(self->m_familyNames);
	CppStdListOfString_delete(self->m_styleNames);
	CppStdListOfString_delete(self->m_fullNames);
	CppStdString_delete(self->m_psName);
	CppStdString_delete(self->m_subFamily);
	free(self);
}

struct XeTeXFontMgr {
	void              (*m_subdtor)(struct XeTeXFontMgr* self);
    void              (*m_memfnInitialize)(struct XeTeXFontMgr* self); /*abstract*/
    void              (*m_memfnTerminate)(struct XeTeXFontMgr* self);
    char*             (*m_memfnGetPlatformFontDesc)(const struct XeTeXFontMgr* self, PlatformFontRef font); /*abstract*/
    void    		  (*m_memfnGetOpSizeRecAndStyleFlags)(struct XeTeXFontMgr* self, XeTeXFontMgrFont* theFont);
    void 			  (*m_memfnSearchForHostPlatformFonts)(struct XeTeXFontMgr* self, const char* name); /* abstract */
    XeTeXFontMgrNameCollection*   (*m_memfnReadNames)(struct XeTeXFontMgr* self, PlatformFontRef fontRef); /* abstract */
};

typedef struct XeTeXFontMgr XeTeXFontMgr;

/*
#include <string>
#include <map>
#include <list>
#include <vector>

class XeTeXFontMgr
{
public:
    static XeTeXFontMgr*            GetFontManager();
        // returns the global fontmanager (creating it if necessary)
    static void                     Terminate();
        // clean up (may be required if using the cocoa implementation)
    static void Destroy();

    PlatformFontRef                 findFont(const char* name, char* variant, double ptSize);
        // 1st arg is name as specified by user (C string, UTF-8)
        // 2nd is /B/I/AAT/OT/ICU/GR/S=## qualifiers
        // 1. try name given as "full name"
        // 2. if there's a hyphen, split and try "family-style"
        // 3. try as PostScript name
        // 4. try name as family with "Regular/Plain/Normal" style
        // apply style qualifiers and optical sizing if present

        // SIDE EFFECT: sets sReqEngine to 'A' or 'O' or 'G' if appropriate,
        //   else clears it to 0

        // SIDE EFFECT: updates TeX variables /nameoffile/ and /namelength/,
        //   to match the actual font found

        // SIDE EFFECT: edits /variant/ string in-place removing /B or /I

    const char*                     getFullName(PlatformFontRef font) const;
        // return the full name of the font, suitable for use in XeTeX source
        // without requiring style qualifiers

    double                          getDesignSize(XeTeXFont font);

    char                            getReqEngine() const { return sReqEngine; }
        // return the requested rendering technology for the most recent findFont
        // or 0 if no specific technology was requested

    void                            setReqEngine(char reqEngine) const { sReqEngine = reqEngine; }

protected:
                                    XeTeXFontMgr()
                                        { }
    virtual                         ~XeTeXFontMgr()
                                        { }

    virtual void                    initialize() = 0;
    virtual void                    terminate();

    virtual char*             		getPlatformFontDesc(PlatformFontRef font) const = 0;


    



    std::map<std::string,Font*>                 m_nameToFont;                     // maps full name (as used in TeX source) to font record
    std::map<std::string,Family*>               m_nameToFamily;
    std::map<PlatformFontRef,Font*>             m_platformRefToFont;
    std::map<std::string,Font*>                 m_psNameToFont;                   // maps PS name (as used in .xdv) to font record

    int             weightAndWidthDiff(const Font* a, const Font* b) const;
    int             styleDiff(const Font* a, int wt, int wd, int slant) const;
    Font*           bestMatchFromFamily(const Family* fam, int wt, int wd, int slant) const;
    void            appendToList(std::list<std::string>* list, const char* str);
    void            prependToList(std::list<std::string>* list, const char* str);
    void            addToMaps(PlatformFontRef platformFont, const NameCollection* names);

    OpSizeRec *getOpSize(XeTeXFont font);

    virtual void    getOpSizeRecAndStyleFlags(Font* theFont);
    virtual void    searchForHostPlatformFonts(const std::string& name) = 0;

    virtual NameCollection*     readNames(PlatformFontRef fontRef) = 0;
};
*/

#endif  /* __XETEX_FONT_MANAGER_H */
