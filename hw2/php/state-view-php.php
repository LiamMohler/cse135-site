<?php
session_start();

header("Cache-Control: no-cache");
header("Content-Type: text/html");

print "<!DOCTYPE html>";
print "<html>";
print "<head><title>PHP State View</title></head>";
print "<body>";
print "<h1>PHP State - View</h1>";

if (isset($_SESSION['username'])) {
    print "<p><strong>Username:</strong> " . $_SESSION['username'] . "</p>";
    print "<p><strong>Favorite Color:</strong> " . $_SESSION['favorite_color'] . "</p>";
    print "<p><strong>Notes:</strong> " . $_SESSION['notes'] . "</p>";
    print "<form method='POST' action='state-clear-php.php'>";
    print "<button type='submit'>Clear</button>";
    print "</form>";
} else {
    print "<p>No data</p>";
}

print "<p><a href='state-php.php'>Back</a></p>";
print "</body></html>";
?>