// overload 0
var $receiver : 4D.DataStore
$receiver:=ds
$receiver.startRequestLog()
// overload 1
$receiver:=ds
var $file1 : 4D.File
$file1:=File("/PACKAGE/README.md")
$receiver.startRequestLog($file1)
// overload 2
$receiver:=ds
var $file2 : 4D.File
$file2:=File("/PACKAGE/README.md")
$receiver.startRequestLog($file2; 1)
// overload 3
$receiver:=ds
$receiver.startRequestLog(1)
