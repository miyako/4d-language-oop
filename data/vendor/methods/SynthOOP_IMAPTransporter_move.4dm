// overload 0
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result1 : Object
$result1:=$receiver.move(New collection; "synthText")
// overload 1
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result2 : Object
$result2:=$receiver.move(IMAP all; "synthText")
