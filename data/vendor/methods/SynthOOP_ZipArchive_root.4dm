// property read
var $receiver : 4D.ZipArchive
$receiver:=ZIP Read archive(File("/PACKAGE/data.zip"))
var $read1 : 4D.ZipFolder
$read1:=$receiver.root
