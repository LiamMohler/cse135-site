<?php
header("Cache-Control: no-cache");
header("Content-Type: text/html");

$form_data = file_get_contents('php://input');

// Parse the POST data
parse_str($form_data, $data);

print "<!DOCTYPE html>";
print "<html><head><title>POST Request Echo</title>";
print "</head><body><h1 align='center'>POST Request Echo</h1>";
print "<hr>";
print "<p><b>Message Body:</b></p>";
print "<ul>\n";

if (!empty($data)) {
    foreach ($data as $key => $value) {
        print "<li>$key = $value</li>\n";
    }
}

print "</ul>\n";
print "</body></html>";
?>
