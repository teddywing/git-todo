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

system('git init t-git-repo');
ok !$?;

copy('t/data/git-sugdiff.rs', 't-git-repo') or die $!;

chdir 't-git-repo' or die $!;

system('git add git-sugdiff.rs');
ok !$?;

system('git commit -m "Commit"');
ok !$?;

done_testing;
