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

bool CppStdString_equal(CppStdString* lhs, CppStdString* rhs);
bool CppStdString_equal_const_char_ptr(CppStdString* lhs, const char* rhs);
bool CppStdString_const_char_ptr_equal_const_char_ptr(const char* lhs, const char* rhs);
CppStdString* CppStdString_clone(CppStdString* self);

void CppStdString_assign_from_const_char_ptr(CppStdString* self, const char* val);
void CppStdString_assign_n_chars(CppStdString* self, const char* val, size_t count);
void CppStdString_append_const_char_ptr(CppStdString* self, const char* val);

const char* CppStdString_cstr(const CppStdString* self);
int CppStdString_length(const CppStdString* self);
char CppStdString_last(const CppStdString* self);

struct CppStdMapStringToFontPtr;
typedef struct CppStdMapStringToFontPtr CppStdMapStringToFontPtr;

struct XeTeXFontMgrFamily {
	CppStdMapStringToFontPtr*    styles;
	uint16_t              minWeight;
	uint16_t              maxWeight;
	uint16_t              minWidth;
	uint16_t              maxWidth;
	int16_t               minSlant;
	int16_t               maxSlant;
};

typedef struct XeTeXFontMgrFamily XeTeXFontMgrFamily;

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


struct CppStdListOfString;
typedef struct CppStdListOfString CppStdListOfString;

CppStdListOfString* CppStdListOfString_create();
void CppStdListOfString_delete(CppStdListOfString* self);

size_t CppStdListOfString_size(const CppStdListOfString* self);
void CppStdListOfString_assign(CppStdListOfString* dest, CppStdListOfString* src);
bool CppStdListOfString_contains_const_char_ptr(const CppStdListOfString* self, const char* val);
void CppStdListOfString_append_copy_CppStdString(CppStdListOfString* list, CppStdString* val);
const char* CppStdListOfString_front_const_char_ptr(const CppStdListOfString* self);

void CppStdListOfString_prepend_copy_const_char_ptr(const CppStdListOfString* self, const char* val);
void CppStdListOfString_append_copy_const_char_ptr(const CppStdListOfString* self, const char* val);

struct CppStdListOfString_Iter {
	void* dummy;
};
typedef struct CppStdListOfString_Iter CppStdListOfString_Iter;
CppStdListOfString_Iter CppStdListOfString_begin(CppStdListOfString* self);
CppStdListOfString_Iter CppStdListOfString_end(CppStdListOfString* self);
void CppStdListOfString_Iter_inc(CppStdListOfString_Iter* iter);
void CppStdListOfString_erase(CppStdListOfString* self, CppStdListOfString_Iter item);
CppStdString* CppStdListOfString_Iter_deref(CppStdListOfString_Iter self);
CppStdString* CppStdString_clone_from_iter(CppStdListOfString_Iter self);

bool CppStdListOfString_Iter_neq(CppStdListOfString_Iter lhs, CppStdListOfString_Iter rhs);

struct XeTeXFontMgrNameCollection {
	CppStdListOfString*  m_familyNames;
	CppStdListOfString*  m_styleNames;
	CppStdListOfString*  m_fullNames;
	CppStdString*        m_psName;
	CppStdString*        m_subFamily;
};
typedef struct XeTeXFontMgrNameCollection XeTeXFontMgrNameCollection;

struct CppStdMapStringToFontPtr;
typedef struct CppStdMapStringToFontPtr CppStdMapStringToFontPtr;

struct CppStdMapStringToFontPtr_Iter {
	void* dummy;
};
typedef struct CppStdMapStringToFontPtr_Iter CppStdMapStringToFontPtr_Iter;


CppStdMapStringToFontPtr* CppStdMapStringToFontPtr_create();
void CppStdMapStringToFontPtr_delete(CppStdMapStringToFontPtr* self);

CppStdMapStringToFontPtr_Iter CppStdMapStringToFontPtr_find(CppStdMapStringToFontPtr* self, CppStdString* val);
CppStdMapStringToFontPtr_Iter CppStdMapStringToFontPtr_find_const_char_ptr(CppStdMapStringToFontPtr* self, const char* val);
void CppStdMapStringToFontPtr_put(CppStdMapStringToFontPtr* self, CppStdString* val, XeTeXFontMgrFont* val2);

CppStdMapStringToFontPtr_Iter CppStdMapStringToFontPtr_begin(CppStdMapStringToFontPtr* self);
CppStdMapStringToFontPtr_Iter CppStdMapStringToFontPtr_end(CppStdMapStringToFontPtr* self);
bool CppStdMapStringToFontPtr_Iter_eq(CppStdMapStringToFontPtr_Iter lhs, CppStdMapStringToFontPtr_Iter rhs);
bool CppStdMapStringToFontPtr_Iter_neq(CppStdMapStringToFontPtr_Iter lhs, CppStdMapStringToFontPtr_Iter rhs);
XeTeXFontMgrFont* CppStdMapStringToFontPtr_Iter_second(CppStdMapStringToFontPtr_Iter self);
void CppStdMapStringToFontPtr_Iter_inc(CppStdMapStringToFontPtr_Iter* iter);


struct CppStdMapStringToFamilyPtr;
typedef struct CppStdMapStringToFamilyPtr CppStdMapStringToFamilyPtr;
CppStdMapStringToFamilyPtr* CppStdMapStringToFamilyPtr_create();
void CppStdMapStringToFamilyPtr_delete(CppStdMapStringToFamilyPtr* self);

struct CppStdMapStringToFamilyPtr_Iter {
	void* dummy;
};
typedef struct CppStdMapStringToFamilyPtr_Iter CppStdMapStringToFamilyPtr_Iter;
XeTeXFontMgrFamily* CppStdMapStringToFamilyPtr_Iter_second(CppStdMapStringToFamilyPtr_Iter self);
bool CppStdMapStringToFamilyPtr_Iter_eq(CppStdMapStringToFamilyPtr_Iter lhs, CppStdMapStringToFamilyPtr_Iter rhs);
bool CppStdMapStringToFamilyPtr_Iter_neq(CppStdMapStringToFamilyPtr_Iter lhs, CppStdMapStringToFamilyPtr_Iter rhs);

CppStdMapStringToFamilyPtr_Iter CppStdMapStringToFamilyPtr_find(CppStdMapStringToFamilyPtr* self, CppStdString* val);
void CppStdMapStringToFamilyPtr_put(CppStdMapStringToFamilyPtr* self, CppStdString* val, XeTeXFontMgrFamily* val2);
CppStdMapStringToFamilyPtr_Iter CppStdMapStringToFamilyPtr_begin(CppStdMapStringToFamilyPtr* self);
CppStdMapStringToFamilyPtr_Iter CppStdMapStringToFamilyPtr_end(CppStdMapStringToFamilyPtr* self);

struct CppStdMapFontRefToFontPtr;
typedef struct CppStdMapFontRefToFontPtr CppStdMapFontRefToFontPtr;
CppStdMapFontRefToFontPtr* CppStdMapFontRefToFontPtr_create();
void CppStdMapFontRefToFontPtr_delete(CppStdMapFontRefToFontPtr* self);
bool CppStdMapFontRefToFontPtr_contains(const CppStdMapFontRefToFontPtr* self, PlatformFontRef val);

struct CppStdMapFontRefToFontPtr_Iter {
	void* dummy;
};
typedef struct CppStdMapFontRefToFontPtr_Iter CppStdMapFontRefToFontPtr_Iter;

CppStdMapFontRefToFontPtr_Iter CppStdMapFontRefToFontPtr_find(CppStdMapFontRefToFontPtr* self, PlatformFontRef val);
void CppStdMapFontRefToFontPtr_put(CppStdMapFontRefToFontPtr* self, PlatformFontRef val, XeTeXFontMgrFont* val2);
CppStdMapFontRefToFontPtr_Iter CppStdMapFontRefToFontPtr_begin(CppStdMapFontRefToFontPtr* self);
CppStdMapFontRefToFontPtr_Iter CppStdMapFontRefToFontPtr_end(CppStdMapFontRefToFontPtr* self);
bool CppStdMapFontRefToFontPtr_Iter_eq(CppStdMapFontRefToFontPtr_Iter lhs, CppStdMapFontRefToFontPtr_Iter rhs);
bool CppStdMapFontRefToFontPtr_Iter_neq(CppStdMapFontRefToFontPtr_Iter lhs, CppStdMapFontRefToFontPtr_Iter rhs);
XeTeXFontMgrFont* CppStdMapFontRefToFontPtr_Iter_second(CppStdMapFontRefToFontPtr_Iter self);


inline XeTeXFontMgrFamily* XeTeXFontMgrFamily_create()
{
	XeTeXFontMgrFamily* self = malloc(sizeof(XeTeXFontMgrFamily));
	self->minWeight = (0);
	self->maxWeight = (0);
	self->minWidth = (0);
	self->maxWidth = (0);
	self->minSlant = (0);
	self->maxSlant = (0);
    self->styles = CppStdMapStringToFontPtr_create();
	return self;
}
inline void XeTeXFontMgrFamily_delete(XeTeXFontMgrFamily* self)
{
	if(!self)
		return;
	CppStdMapStringToFontPtr_delete(self->styles);
	free(self);
}

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
    CppStdMapStringToFontPtr*                 m_nameToFont;                     // maps full name (as used in TeX source) to font record
    CppStdMapStringToFamilyPtr*               m_nameToFamily;
    CppStdMapFontRefToFontPtr*                m_platformRefToFont;
    CppStdMapStringToFontPtr*                 m_psNameToFont;                   // maps PS name (as used in .xdv) to font record
};

typedef struct XeTeXFontMgr XeTeXFontMgr;

XeTeXFontMgr* XeTeXFontMgr_GetFontManager();
void XeTeXFontMgr_Terminate();
void XeTeXFontMgr_Destroy();

void XeTeXFontMgr_base_ctor(XeTeXFontMgr* self);

inline void XeTeXFontMgr_initialize(XeTeXFontMgr* self) {
	self->m_memfnInitialize(self);
}

inline void XeTeXFontMgr_terminate(XeTeXFontMgr* self) {
	self->m_memfnTerminate(self);
}

inline char* XeTeXFontMgr_getPlatformFontDesc(const XeTeXFontMgr* self, PlatformFontRef font) {
	return self->m_memfnGetPlatformFontDesc(self, font);
}

inline void XeTeXFontMgr_searchForHostPlatformFonts(struct XeTeXFontMgr* self, const char* name) {
	self->m_memfnSearchForHostPlatformFonts(self, name);
}

inline void XeTeXFontMgr_getOpSizeRecAndStyleFlags(struct XeTeXFontMgr* self, XeTeXFontMgrFont* theFont) {
	self->m_memfnGetOpSizeRecAndStyleFlags(self, theFont);
}

inline XeTeXFontMgrNameCollection* XeTeXFontMgr_readNames(struct XeTeXFontMgr* self, PlatformFontRef fontRef) {
    return self->m_memfnReadNames(self, fontRef); 
}
void XeTeXFontMgr_delete(XeTeXFontMgr* self);

void
XeTeXFontMgr_appendToList(XeTeXFontMgr* self, CppStdListOfString* list, const char* str);
void
XeTeXFontMgr_prependToList(XeTeXFontMgr* self, CppStdListOfString* list, const char* str);

void
XeTeXFontMgr_addToMaps(XeTeXFontMgr* self, PlatformFontRef platformFont, const XeTeXFontMgrNameCollection* names);


void XeTeXFontMgr_base_getOpSizeRecAndStyleFlags(XeTeXFontMgr* self, XeTeXFontMgrFont* theFont);
XeTeXFontMgrFont* XeTeXFontMgr_bestMatchFromFamily(const XeTeXFontMgr* self, const XeTeXFontMgrFamily* fam, int wt, int wd, int slant);
int XeTeXFontMgr_weightAndWidthDiff(const XeTeXFontMgr* self, const XeTeXFontMgrFont* a, const XeTeXFontMgrFont* b);

PlatformFontRef
XeTeXFontMgr_findFont(XeTeXFontMgr* self, const char* name, char* variant, double ptSize);

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
