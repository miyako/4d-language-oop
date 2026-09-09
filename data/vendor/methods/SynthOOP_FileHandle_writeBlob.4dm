// overload 0
var $receiver : 4D.FileHandle
$receiver:=File("/PACKAGE/data.txt").open("write")
var $blob1 : 4D.Blob
$blob1:=4D.Blob.new()
$receiver.writeBlob($blob1)
