<?php
session_start();

if ($_SERVER['REQUEST_METHOD'] == 'POST') {
    $_SESSION['username'] = $_POST['username'];
    $_SESSION['favorite_color'] = $_POST['favorite_color'];
    $_SESSION['notes'] = $_POST['notes'];
    header("Location: state-view-php.php");
    exit();
}

print "Cache-Control: no-cache\n";
print "Content-Type: text/html\n\n";

print "<!DOCTYPE html>";
print "<html>";
print "<head><title>PHP State Input</title></head>";
print "<body>";
print "<h1>PHP State - Input</h1>";
print "<form method='POST'>";
print "<label>Username:</label><br>";
print "<input type='text' name='username'><br><br>";
print "<label>Favorite Color:</label><br>";
print "<input type='text' name='favorite_color'><br><br>";
print "<label>Notes:</label><br>";
print "<textarea name='notes' rows='4'></textarea><br><br>";
print "<button type='submit'>Save</button>";
print "</form>";
print "<p><a href='state-view-php.php'>View Data</a></p>";
print "</body></html>";
?>