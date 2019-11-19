midlrt /metadata_dir %WINDIR%\System32\WinMetadata /h nul /nomidl /winrt /winmd metadata.winmd /reference %WINDIR%\System32\WinMetadata\Windows.Foundation.winmd metadata.idl
mdmerge metadata.winmd
