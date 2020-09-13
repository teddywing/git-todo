#!/usr/bin/env perl -w

use strict;

use Test::More;

if (!-d 't-git-repo') {
	plan skip_all => 'Testing stage already cleaned.';
}

system('rm -rf t-git-repo');
ok !$?;

done_testing;
