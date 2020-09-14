#!/usr/bin/env perl -w

use strict;

use File::Copy;
use Test::More;

use Bin qw($BIN);

my $file = 'git-sugdiff.rs';

chdir 't-git-repo' or die $!;

system('git checkout -b fork-point');
ok !$?;

open(my $input, '<', $file) or die $!;
open(my $output, '>', "$file.out") or die $!;

while (<$input>) {
	if ($. == 34) {
		print $output "    // TODO: 100-shows-todo-comments-since-fork-point\n";
	}

	print $output $_;
}

close $input;
close $output;

move("$file.out", $file) or die $!;

system('git add git-sugdiff.rs');
ok !$?;

system('git commit -m "New TODO"');
ok !$?;

my $todos = qx($BIN);
is $todos, 'TODO: 100-shows-todo-comments-since-fork-point
';


# Teardown
system('git branch -d fork-point');


done_testing;