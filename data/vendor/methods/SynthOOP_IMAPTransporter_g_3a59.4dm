// overload 0
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result1 : Object
$result1:=$receiver.getMail(1; New object)
// overload 0 [optional:omit-options]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result2 : Object
$result2:=$receiver.getMail(1)
// overload 1
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result3 : Object
$result3:=$receiver.getMail("synthText"; New object)
// overload 1 [optional:omit-options]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result4 : Object
$result4:=$receiver.getMail("synthText")
