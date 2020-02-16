#ifndef _PASS_DEFS_H

void partial_eval(ASTNode *node);
void uniquify(ASTNode *node);
ASTNode *flattern(ASTNode *root);
void select_inst(ASTNode *root);
void assign_homes(ASTNode *root);
void patch_inst(ASTNode *root);

#define _PASS_DEFS_H
#endif
