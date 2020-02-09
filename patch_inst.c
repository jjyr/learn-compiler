/* fix the invalid instructures */

#include "ast.h"
#include "defs.h"
#include "error.h"
#include "table.h"
#include <stdio.h>
#include <string.h>

void patch_inst(ASTNode *node) {
  while (node != 0) {
    switch (node->token) {
    case MOVQ:
    case ADDQ:
      if (node->lhs->token == STACK_LOC) {
        /* need patch, use RAX as temp register */
        ASTNode *reg = alloc_node();
        reg->token = REG;
        reg->value = RAX;
        /* copy node operation to t, except the lhs */
        ASTNode *t = alloc_node();
        t->token = node->token;
        t->lhs = reg;
        t->value = node->value;
        /* change node operator to MOVQ */
        node->token = MOVQ;
        node->value = (size_t)reg;
        /* insert t after this operation */
        t->rhs = node->rhs;
        node->rhs = t;
        node = node->rhs;
      }
    }
    node = node->rhs;
  }
}
