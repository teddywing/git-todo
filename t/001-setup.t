#!/usr/bin/env perl -w

use strict;

use File::Copy;
use Test::More;

system('git init t-git-repo');
ok !$?;

copy('t/data/git-sugdiff.rs', 't-git-repo') or die $!;

chdir 't-git-repo' or die $!;

system('git add git-sugdiff.rs');
ok !$?;

system('git commit -m "Commit"');
ok !$?;

done_testing;
