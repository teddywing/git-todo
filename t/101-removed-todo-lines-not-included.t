#!/usr/bin/env perl -w

# Copyright (c) 2020  Teddy Wing
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.


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
	if ($_ eq "// TODO: one\n") {
		next;
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
is $todos, '';


# Teardown
system('git checkout master');
system('git branch -D fork-point');


done_testing;
