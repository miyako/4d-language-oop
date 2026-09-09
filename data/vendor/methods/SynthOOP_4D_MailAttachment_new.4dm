// overload 0
var $file1 : 4D.File
$file1:=File("/PACKAGE/README.md")
var $result2 : 4D.MailAttachment
$result2:=4D.MailAttachment.new($file1; "synthText"; "synthText"; "synthText"; "synthText")
// overload 0 [optional:omit-name]
var $file3 : 4D.File
$file3:=File("/PACKAGE/README.md")
var $result4 : 4D.MailAttachment
$result4:=4D.MailAttachment.new($file3)
// overload 0 [optional:omit-cid]
var $file5 : 4D.File
$file5:=File("/PACKAGE/README.md")
var $result6 : 4D.MailAttachment
$result6:=4D.MailAttachment.new($file5; "synthText")
// overload 0 [optional:omit-type]
var $file7 : 4D.File
$file7:=File("/PACKAGE/README.md")
var $result8 : 4D.MailAttachment
$result8:=4D.MailAttachment.new($file7; "synthText"; "synthText")
// overload 0 [optional:omit-disposition]
var $file9 : 4D.File
$file9:=File("/PACKAGE/README.md")
var $result10 : 4D.MailAttachment
$result10:=4D.MailAttachment.new($file9; "synthText"; "synthText"; "synthText")
// overload 1
var $zipFile11 : 4D.ZipFile
var $zipArchive : 4D.ZipArchive
$zipArchive:=ZIP Read archive(File("/PACKAGE/data.zip"))
$zipFile11:=$zipArchive.root.files()[0]
var $result12 : 4D.MailAttachment
$result12:=4D.MailAttachment.new($zipFile11; "synthText"; "synthText"; "synthText"; "synthText")
// overload 1 [optional:omit-name]
var $zipFile13 : 4D.ZipFile
$zipArchive:=ZIP Read archive(File("/PACKAGE/data.zip"))
$zipFile13:=$zipArchive.root.files()[0]
var $result14 : 4D.MailAttachment
$result14:=4D.MailAttachment.new($zipFile13)
// overload 1 [optional:omit-cid]
var $zipFile15 : 4D.ZipFile
$zipArchive:=ZIP Read archive(File("/PACKAGE/data.zip"))
$zipFile15:=$zipArchive.root.files()[0]
var $result16 : 4D.MailAttachment
$result16:=4D.MailAttachment.new($zipFile15; "synthText")
// overload 1 [optional:omit-type]
var $zipFile17 : 4D.ZipFile
$zipArchive:=ZIP Read archive(File("/PACKAGE/data.zip"))
$zipFile17:=$zipArchive.root.files()[0]
var $result18 : 4D.MailAttachment
$result18:=4D.MailAttachment.new($zipFile17; "synthText"; "synthText")
// overload 1 [optional:omit-disposition]
var $zipFile19 : 4D.ZipFile
$zipArchive:=ZIP Read archive(File("/PACKAGE/data.zip"))
$zipFile19:=$zipArchive.root.files()[0]
var $result20 : 4D.MailAttachment
$result20:=4D.MailAttachment.new($zipFile19; "synthText"; "synthText"; "synthText")
// overload 2
var $blob21 : 4D.Blob
$blob21:=4D.Blob.new()
var $result22 : 4D.MailAttachment
$result22:=4D.MailAttachment.new($blob21; "synthText"; "synthText"; "synthText"; "synthText")
// overload 2 [optional:omit-name]
var $blob23 : 4D.Blob
$blob23:=4D.Blob.new()
var $result24 : 4D.MailAttachment
$result24:=4D.MailAttachment.new($blob23)
// overload 2 [optional:omit-cid]
var $blob25 : 4D.Blob
$blob25:=4D.Blob.new()
var $result26 : 4D.MailAttachment
$result26:=4D.MailAttachment.new($blob25; "synthText")
// overload 2 [optional:omit-type]
var $blob27 : 4D.Blob
$blob27:=4D.Blob.new()
var $result28 : 4D.MailAttachment
$result28:=4D.MailAttachment.new($blob27; "synthText"; "synthText")
// overload 2 [optional:omit-disposition]
var $blob29 : 4D.Blob
$blob29:=4D.Blob.new()
var $result30 : 4D.MailAttachment
$result30:=4D.MailAttachment.new($blob29; "synthText"; "synthText"; "synthText")
// overload 3
var $result31 : 4D.MailAttachment
$result31:=4D.MailAttachment.new("synthText"; "synthText"; "synthText"; "synthText"; "synthText")
// overload 3 [optional:omit-name]
var $result32 : 4D.MailAttachment
$result32:=4D.MailAttachment.new("synthText")
// overload 3 [optional:omit-cid]
var $result33 : 4D.MailAttachment
$result33:=4D.MailAttachment.new("synthText"; "synthText")
// overload 3 [optional:omit-type]
var $result34 : 4D.MailAttachment
$result34:=4D.MailAttachment.new("synthText"; "synthText"; "synthText")
// overload 3 [optional:omit-disposition]
var $result35 : 4D.MailAttachment
$result35:=4D.MailAttachment.new("synthText"; "synthText"; "synthText"; "synthText")
