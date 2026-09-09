// overload 0
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result1 : Collection
$result1:=$receiver.getBoxList(New object)
// overload 0 [optional:omit-parameters]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result2 : Collection
$result2:=$receiver.getBoxList()
