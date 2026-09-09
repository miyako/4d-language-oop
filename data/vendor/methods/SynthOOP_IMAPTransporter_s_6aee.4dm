// overload 0
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result1 : Object
$result1:=$receiver.selectBox("synthText"; IMAP read only state)
// overload 0 [enum IMAPTransporter.selectBox.state = IMAP read write state]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result2 : Object
$result2:=$receiver.selectBox("synthText"; IMAP read write state)
// overload 0 [optional:omit-state]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result3 : Object
$result3:=$receiver.selectBox("synthText")
