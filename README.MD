Make a proof-checker.

Language is based on INF402 lessons:

# The file format
The file is a succession of records and comments.

## Records
Each record consists of 4 blocks separated by `;` :
    1) The current number of the block
    2) The current context of the block
    3) A statement
    4) A justification
### Context
Context is a list of block numbers, separated by `,`.
### Statement
A statement may or may not begin with `Supposons ` or `Donc `.
Then comes a logical formula, composed by:
    * Variable name (for now single letter, not `v` or `T`)
    * `T` or `_`
    * `-`_`formula`_
    * `(`_`formula`_`)`
    * _`formula`_`^|v|=>|<=|<=>`_`formula`_
### Justification
A justification might be one of the following:
    * ` `, for `Supposons ` or `Donc ` lines
    * `
## Comments
Comments start by `(*` and are ended by `*)`.