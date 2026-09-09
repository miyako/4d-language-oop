// overload 0
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result1 : Object
$result1:=$receiver.getMails(New collection; New object)
// overload 0 [optional:omit-options]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result2 : Object
$result2:=$receiver.getMails(New collection)
// overload 1
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result3 : Object
$result3:=$receiver.getMails(1; 1; New object)
// overload 1 [optional:omit-options]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result4 : Object
$result4:=$receiver.getMails(1; 1)
