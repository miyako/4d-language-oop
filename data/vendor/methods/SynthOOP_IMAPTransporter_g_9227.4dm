// overload 0
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result1 : Object
$result1:=$receiver.getBoxInfo("synthText")
// overload 0 [optional:omit-name]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result2 : Object
$result2:=$receiver.getBoxInfo()
