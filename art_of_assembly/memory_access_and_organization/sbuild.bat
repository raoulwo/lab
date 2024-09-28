echo off

ml64 /nologo /c /Zi /Cp %1.asm
cl /nologo /O2 /Zi /utf-8 /EHa /Fe%1.exe generic_driver.cpp %1.obj /link /largeaddressaware:no
