// overload 0
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result1 : Blob
$result1:=$receiver.getMIMEAsBlob(1; True)
// overload 0 [optional:omit-updateSeen]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result2 : Blob
$result2:=$receiver.getMIMEAsBlob(1)
// overload 1
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result3 : Blob
$result3:=$receiver.getMIMEAsBlob("synthText"; True)
// overload 1 [optional:omit-updateSeen]
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $result4 : Blob
$result4:=$receiver.getMIMEAsBlob("synthText")
