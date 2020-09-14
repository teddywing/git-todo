package Bin;

use strict;
use warnings;

use Exporter qw(import);
our @EXPORT = qw($BIN);

use File::Spec;

# our $BIN = File::Spec->rel2abs('git-todo');
our $BIN = File::Spec->rel2abs('./target/debug/git-todo');

1;
