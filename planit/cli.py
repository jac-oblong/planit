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
Handles all CLI argument parsing
"""


import argparse


def parse_cli(argv: list[str]) -> argparse.Namespace:
    """
    Parses the arguments provided

    :param argv: list of command line arguments to be parsed
    :return: argparse Namespace containing parsed information
    """

    parser = argparse.ArgumentParser(
        prog="PlanIt", description="A project management tool"
    )
    commands = parser.add_subparsers(dest="command", required=False)

    ############################################################################
    # List tasks                                                               #
    ############################################################################
    list = commands.add_parser("list", aliases=["ls"], help="list tasks")

    ############################################################################
    # Edit tasks                                                               #
    ############################################################################
    add = commands.add_parser("add", help="edit tasks")

    return parser.parse_args(argv)
