#!/usr/bin/env python

################################################################################
#                                                                              #
# The MIT License (MIT)                                                        #
#                                                                              #
# Copyright (c) 2025 Jacob Long                                                #
#                                                                              #
# Permission is hereby granted, free of charge, to any person obtaining        #
# a copy of this software and associated documentation files (the              #
# "Software"), to deal in the Software without restriction, including          #
# without limitation the rights to use, copy, modify, merge, publish,          #
# distribute, sublicense, and/or sell copies of the Software, and to           #
# permit persons to whom the Software is furnished to do so, subject to        #
# the following conditions:                                                    #
#                                                                              #
# The above copyright notice and this permission notice shall be               #
# included in all copies or substantial portions of the Software.              #
#                                                                              #
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,              #
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF           #
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.       #
# IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY         #
# CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,         #
# TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE            #
# SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.                       #
#                                                                              #
################################################################################


"""
Where it all starts
"""


import sys

import cli


def main(argv: list[str]):
    """
    Based on the argument list provided, calls the correct function

    :param argv: list of command line arguments to be parsed
    :return: None
    """

    args = cli.parse_cli(argv)

    if args.command == "list" or args.command == "ls" or args.command == None:
        print("All Todos:")
    elif args.command == "add":
        assert False, "Adding items not supported"
    else:
        assert False, "Unreachable"


if __name__ == "__main__":
    main(sys.argv[1:])
