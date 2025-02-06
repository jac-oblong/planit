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
Handles Task related processes
"""


from datetime import datetime


class Task:
    """
    The basic unit of work, everything should be boiled down to task(s)
    """

    def __init__(
        self,
        id: int,
        name: str,
        start: datetime = None,
        end: datetime = None,
        repeat: str = None,
        tags: list[str] = [],
        **kwargs,
    ):
        """
        Creates a Task

        :param id: The unique id number for identifying this task
        :param name: The name to be displayed
        :param start: The start date
        :param end: The end data
        :param repeat: How often the task should repeat
        :param tags: Tags for the task
        :param kwargs: Any other custom attributes for task
        """

        self.id = id
        self.name = name
        self.start = start
        self.end = end
        self.repeat = repeat
        self.tags = tags
        self.attributes = kwargs
