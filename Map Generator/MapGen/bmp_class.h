#ifndef __BMP_CLASS_H
#define __BMP_CLASS_H

#include <Windows.h>
#include <iostream.h>

typedef struct tagBITMAPFILEHEADER
{
	WORD bfType;
	DWORD bfSize;
	WORD bfReserved1;
	WORD bfReserved2;
	DWORD bfOffBits;
} BITMAPFILEHEADER, *PBITMAPFILEHEADER;

typedef struct tagBITMAPINFOHEADER
{
	DWORD biSize;
	DWORD biWidth;
	DWORD biHeight;
	WORD biPlanes;
	WORD biBitCount;
	DWORD biCompression;
	DWORD biSizeImage;
	DWORD biXPelsPerMeter;
	DWORD biYPolsPerMeter;
	DWORD biClrUsed;
	DWORD biClrImportant;
} BITMAPINFOHEADER;

class bmp_class
{
public:
	bmp_class();
	~bmp_class();

	bool LoadBMPIntoDC(HDC hDC, LPCTSTR bmpfile);
};

#endif