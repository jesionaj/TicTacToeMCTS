# TicTacToeMCTS
An implementation of Monte-Carlo Tree Search in Rust to "solve" Tic-Tac-Toe.

Recently I read (Google’s paper on AlphaGo)[https://gogameguru.com/i/2016/03/deepmind-mastering-go.pdf]. One of the most interesting techniques they describe in that paper is the Monte-Carlo Tree Search, or MCTS. The basic idea of MCTS is to use many Monte-Carlo simulations to evaluate the strength of a certain move. By playing many, many games we can evaluate the strength of a move based on how many times it wins us a game versus how many times it loses it.

What’s exciting about MCTS is it does not require an evaluation function to work, that is, it doesn’t need a way of figuring out if a given game state is more or less favorable. Instead, implementing the rules is all that’s needed to start using MCTS!

I decided to implement a simple Tic-Tac-Toe MCTS as an exercise in learning Rust. Before starting, I made the following hypotheses about what I’d see:
*	Player 1 taking the center will be the best opening move  
*	Most games will end in a draw  
*	But, Player 1 will win much more often than Player 2  

So what were the results?

> P1 Wins: 11696471 P2 Wins: 5764003 Draws: 2539526 - Total 20000000  
> 0: P1 Wins: 1349158 P2 Wins: 586863 Draws: 286107 - Total 2222128  
> 1: P1 Wins: 1191181 P2 Wins: 747129 Draws: 285073 - Total 2223383  
> 2: P1 Wins: 1349563 P2 Wins: 586821 Draws: 286067 - Total 2222451  
> 3: P1 Wins: 1190575 P2 Wins: 747105 Draws: 286771 - Total 2224451  
> 4: P1 Wins: __1539574__ P2 Wins: 429345 Draws: 253560 - Total 2222479  
> 5: P1 Wins: 1189188 P2 Wins: 746233 Draws: 285444 - Total 2220865  
> 6: P1 Wins: 1347026 P2 Wins: 587329 Draws: 286448 - Total 2220803  
> 7: P1 Wins: 1190695 P2 Wins: 746350 Draws: 284860 - Total 2221905  
> 8: P1 Wins: 1349511 P2 Wins: 586828 Draws: 285196 - Total 2221535  

Where the board positions are represented by:
> 0 | 1 | 2  
> 3 | 4 | 5  
> 6 | 7 | 8  

Immediately we can see that the data shows two of my three hypothesis correct and one very wrong! First, the center position is the winningest position for Player 1 to start on, followed by the corners, and finishing with the edges. Second, Player 1 wins about twice as often as Player 2. However, draws only occur in about 17% of games! While we know that a perfectly played game of Tic-Tac-Toe will always end in a draw, that conclusion is not evident from the top node’s data. In order to come to that conclusion, we have to actually play a game.

Unlike our MCTS, our AI does have to have some evaluation function to determine which move to make. This became immediately obvious after running the AI using the evaluation function:
> Value(node) = wins + (draws * 0.5)

Here’s the result:

> P1 Wins: 11696776 P2 Wins: 5761815 Draws: 2541409 - Total 20000000  
> 0: P1 Wins: 1349573 P2 Wins: 585449 Draws: 286134 - Total 2221156  
> 1: P1 Wins: 1191803 P2 Wins: 745926 Draws: 286125 - Total 2223854  
> 2: P1 Wins: 1346050 P2 Wins: 588142 Draws: 285412 - Total 2219604  
> 3: P1 Wins: 1190629 P2 Wins: 746125 Draws: 285790 - Total 2222544  
> __4: P1 Wins: 1539802 P2 Wins: 427464 Draws: 253458 - Total 2220724__  
> 5: P1 Wins: 1191514 P2 Wins: 746885 Draws: 286576 - Total 2224975  
> 6: P1 Wins: 1348153 P2 Wins: 588462 Draws: 286226 - Total 2222841  
> 7: P1 Wins: 1189706 P2 Wins: 745778 Draws: 286024 - Total 2221508  
> 8: P1 Wins: 1349546 P2 Wins: 587584 Draws: 285664 - Total 2222794  
> Player1: Next node chosen: 4  
> 0: P1 Wins: 182454 P2 Wins: 63248 Draws: 31665 - Total 277367  
> 1: P1 Wins: 201914 P2 Wins: 43475 Draws: 31749 - Total 277138  
> __2: P1 Wins: 182672 P2 Wins: 63469 Draws: 31478 - Total 277619__  
> 3: P1 Wins: 202675 P2 Wins: 43724 Draws: 31807 - Total 278206  
> 5: P1 Wins: 202516 P2 Wins: 43589 Draws: 31715 - Total 277820  
> 6: P1 Wins: 182998 P2 Wins: 63365 Draws: 31481 - Total 277844  
> 7: P1 Wins: 202730 P2 Wins: 43639 Draws: 31796 - Total 278165  
> 8: P1 Wins: 181843 P2 Wins: 62955 Draws: 31767 - Total 276565  
> Player2: Next node chosen: 2  
> 0: P1 Wins: 28378 P2 Wins: 7432 Draws: 3918 - Total 39728  
> __1: P1 Wins: 27354 P2 Wins: 6642 Draws: 6042 - Total 40038__  
> 3: P1 Wins: 26377 P2 Wins: 11630 Draws: 1895 - Total 39902  
> 5: P1 Wins: 27083 P2 Wins: 6581 Draws: 5863 - Total 39527  
> 6: P1 Wins: 19272 P2 Wins: 12435 Draws: 7842 - Total 39549  
> 7: P1 Wins: 26129 P2 Wins: 11326 Draws: 2005 - Total 39460  
> 8: P1 Wins: 28079 P2 Wins: 7423 Draws: 3913 - Total 39415  
> Player1: Next node chosen: 1  
> 0: P1 Wins: 4796 P2 Wins: 439 Draws: 1359 - Total 6594  
> 3: P1 Wins: 4940 P2 Wins: 414 Draws: 1356 - Total 6710  
> 5: P1 Wins: 4366 P2 Wins: 1666 Draws: 674 - Total 6706  
> 6: P1 Wins: 5695 P2 Wins: 940 Draws: 0 - Total 6635  
> 7: P1 Wins: 3624 P2 Wins: 1106 Draws: 2012 - Total 6742  
> __8: P1 Wins: 3933 P2 Wins: 2077 Draws: 641 - Total 6651__ <-- Wrong move!  
> Player2: Next node chosen: 8  
> 0: P1 Wins: 327 P2 Wins: 724 Draws: 208 - Total 1259  
> 3: P1 Wins: 630 P2 Wins: 689 Draws: 0 - Total 1319  
> 5: P1 Wins: 1014 P2 Wins: 115 Draws: 214 - Total 1343  
> 6: P1 Wins: 560 P2 Wins: 549 Draws: 219 - Total 1328  
> __7: P1 Wins: 1402 P2 Wins: 0 Draws: 0 - Total 1402__  
> Player1: Next node chosen: 7  
> Player1 wins!  

The AI actually loses if we do that (Both Player 1 and Player 2 are controlled by the same logic)! We’ve valued wins so highly that the AI takes the move more likely to end in a win (as it sets itself up for a three-in-a-row next turn), without taking into account that it lets Player 1 easily win. Let’s adjust the value function:

Value(node) = wins + draws

And run it again:
> P1 Wins: 11696471 P2 Wins: 5764003 Draws: 2539526 - Total 20000000  
> 0: P1 Wins: 1349158 P2 Wins: 586863 Draws: 286107 - Total 2222128  
> 1: P1 Wins: 1191181 P2 Wins: 747129 Draws: 285073 - Total 2223383  
> 2: P1 Wins: 1349563 P2 Wins: 586821 Draws: 286067 - Total 2222451  
> 3: P1 Wins: 1190575 P2 Wins: 747105 Draws: 286771 - Total 2224451  
> __4: P1 Wins: 1539574 P2 Wins: 429345 Draws: 253560 - Total 2222479__  
> 5: P1 Wins: 1189188 P2 Wins: 746233 Draws: 285444 - Total 2220865  
> 6: P1 Wins: 1347026 P2 Wins: 587329 Draws: 286448 - Total 2220803  
> 7: P1 Wins: 1190695 P2 Wins: 746350 Draws: 284860 - Total 2221905  
> 8: P1 Wins: 1349511 P2 Wins: 586828 Draws: 285196 - Total 2221535  
> Player1: Next node chosen: 4  
> 0: P1 Wins: 182777 P2 Wins: 63634 Draws: 31780 - Total 278191  
> 1: P1 Wins: 202509 P2 Wins: 43903 Draws: 31652 - Total 278064  
> __2: P1 Wins: 182277 P2 Wins: 63593 Draws: 31850 - Total 277720__  
> 3: P1 Wins: 201485 P2 Wins: 43611 Draws: 31682 - Total 276778  
> 5: P1 Wins: 202610 P2 Wins: 43866 Draws: 31669 - Total 278145  
> 6: P1 Wins: 182788 P2 Wins: 63916 Draws: 31850 - Total 278554  
> 7: P1 Wins: 202599 P2 Wins: 43447 Draws: 31570 - Total 277616  
> 8: P1 Wins: 182529 P2 Wins: 63375 Draws: 31507 - Total 277411  
> Player2: Next node chosen: 2  
> 0: P1 Wins: 28144 P2 Wins: 7455 Draws: 3979 - Total 39578  
> 1: P1 Wins: 26874 P2 Wins: 6676 Draws: 5882 - Total 39432  
> 3: P1 Wins: 26161 P2 Wins: 11529 Draws: 1988 - Total 39678  
> __5: P1 Wins: 26898 P2 Wins: 6616 Draws: 6148 - Total 39662__  
> 6: P1 Wins: 19466 P2 Wins: 12224 Draws: 7861 - Total 39551  
> 7: P1 Wins: 26332 P2 Wins: 11642 Draws: 2003 - Total 39977  
> 8: P1 Wins: 28402 P2 Wins: 7451 Draws: 3989 - Total 39842  
> Player1: Next node chosen: 5  
> 0: P1 Wins: 3824 P2 Wins: 2121 Draws: 694 - Total 6639  
> 1: P1 Wins: 4248 P2 Wins: 1692 Draws: 687 - Total 6627  
> __3: P1 Wins: 3519 P2 Wins: 1053 Draws: 2095 - Total 6667__  
> 6: P1 Wins: 5663 P2 Wins: 870 Draws: 0 - Total 6533  
> 7: P1 Wins: 4859 P2 Wins: 437 Draws: 1327 - Total 6623  
> 8: P1 Wins: 4785 P2 Wins: 443 Draws: 1345 - Total 6573  
> Player2: Next node chosen: 3  
> __0: P1 Wins: 855 P2 Wins: 0 Draws: 458 - Total 1313__  
> 1: P1 Wins: 789 P2 Wins: 106 Draws: 458 - Total 1353  
> 6: P1 Wins: 471 P2 Wins: 197 Draws: 715 - Total 1383  
> 7: P1 Wins: 759 P2 Wins: 303 Draws: 242 - Total 1304  
> 8: P1 Wins: 645 P2 Wins: 447 Draws: 222 - Total 1314  
> Player1: Next node chosen: 0  
> 1: P1 Wins: 219 P2 Wins: 0 Draws: 113 - Total 332  
> 6: P1 Wins: 311 P2 Wins: 0 Draws: 0 - Total 311  
> 7: P1 Wins: 220 P2 Wins: 0 Draws: 115 - Total 335    
> __8: P1 Wins: 105 P2 Wins: 0 Draws: 230 - Total 335__  
> Player2: Next node chosen: 8  
> __1: P1 Wins: 41 P2 Wins: 0 Draws: 58 - Total 99__  
> 6: P1 Wins: 0 P2 Wins: 0 Draws: 115 - Total 115  
> 7: P1 Wins: 64 P2 Wins: 0 Draws: 57 - Total 121  
> Player1: Next node chosen: 1  
> 6: P1 Wins: 41 P2 Wins: 0 Draws: 0 - Total 41  
> __7: P1 Wins: 0 P2 Wins: 0 Draws: 58 - Total 58__  
> Player2: Next node chosen: 7  
> __6: P1 Wins: 0 P2 Wins: 0 Draws: 58 - Total 58__  
> Player1: Next node chosen: 6  
> Result: Draw  

Phew, there’s the behavior we hope to see. The final board looks like this:  
  X | X | O  
  O | X | X  
  X | O | O  


Note there are some naive elements in my MCTS still -- each node in the tree is the result of a sequence of moves instead of a board state itself. Further, boards being rotations or reflections of each other are not taken into account. This would be the next step to improve this code.

