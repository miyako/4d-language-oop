// overload 0
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
$receiver.setText("synthText"; "synthText"; 1)
// overload 0 [optional:omit-charSetName]
$receiver:=File("/PACKAGE/README.md")
$receiver.setText("synthText")
// overload 0 [optional:omit-breakMode]
$receiver:=File("/PACKAGE/README.md")
$receiver.setText("synthText"; "synthText")
// overload 1
$receiver:=File("/PACKAGE/README.md")
$receiver.setText("synthText"; 1; 1)
// overload 1 [optional:omit-charSetNum]
$receiver:=File("/PACKAGE/README.md")
$receiver.setText("synthText")
// overload 1 [optional:omit-breakMode]
$receiver:=File("/PACKAGE/README.md")
$receiver.setText("synthText"; 1)
