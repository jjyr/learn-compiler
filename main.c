#include "ast.h"
#include "parser.h"

// parse racket to ast
ASTNode *parse_ast(char *source) {
  Parser p;
  init_parser(&p, source);
  return parse_program(&p);
}

// partial evaluate root
int partial_eval(ASTNode *root) {
  // TODO
  return 1;
}

void print_ast(AstNode *node) {
  // TODO
}

int main() {
  char s[] = "(+ (read) (- (+ 5 3)))";
  ASTNode *root = parse_ast(s);
  partial_eval(root);
  return 0;
}
