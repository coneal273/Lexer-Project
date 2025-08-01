Homework 2 Solution: http://gitlab.cse.lehigh.edu/cse262-programming-languages/spring-2024/assignments/homework-2/-/tree/solutions?ref_type=heads

# CSE262 - Programming Languages - Spring 2024

# Midterm Exam

⏰ **Due by: 3/23/2024 EOD**

## Ethics Contract

**FIRST**: Please read the following carefully:

- I am the sole author of the content within this exam unless otherwise cited.
- I am not an uncredited author of any content in any other exam.
- I will not dicsuss the exam until after the submission deadline.
- All resources I used (including text books and online references, websites, language models), are cited in this exam.
- I will not plagiarize someone else's work and turn it in as my own. If I use someone else's work in this exam, I will cite that work. Failure to cite work I used is plagiarism.
- I understand that acts of academic dishonesty may be penalized to the full extent allowed by the [Lehigh University Code of Conduct][0], including receiving a failing grade for the course. I recognize that I am responsible for understanding the provisions of the Lehigh University Code of Conduct as they relate to this academic exercise.


If you agree with the above, type your full name next to the pen emoji, along with the date. Your exam **will not be graded** without this assent.

---------------------------------------------
🖋️Christian E. O'Neal
---------------------------------------------

💥 **IMPORTANT:** When you are done, make your first commit with the commit message: `I, <your full name here>, agree to the ethics contract`.

💥 **IMPORTANT: As you are working on your midterm, commit your progress regularly.**

## Exam

## Part 1 (10 pts)

Consider the following program source code written in an unknown language:

```
fn foo(a,b,c) {
    let x = a + 1; 
    // This is a comment
    let y = bar(c - b);
    return x * y; // Add the results
}

fn bar(a) {
    return a - 3;
}

fn main() {
	return foo(123,56,7.89);  
}
```

Write a lexer that is capable of tokenizing this source code. You can ignore whitespace. The tokens must be from the following variants:

Each token should have a lexeme (it doesn't have to have start and end row/col)

```
- Keywords:
-    fn
-    let
-    return
-Operator  
-    plus
-    minus
-    divide
-    multiply
- Semicolon
-Identifier
-Integer
-Float
-Comment
-Equal
-OpenCurly
-CloseCurly
-OpenParen
-CloseParen
-Other (any other chacater)
```

## Part 2 (10 pts)

- Extend the source code in some way by adding new syntax.
- Add additional token types to accomodate your extension.
- Extend your lexer to tokenize the new source code.

You can do whatever you want, but make sure you add at least two more token variants.

Some suggestions:

- Add multiline comments
- Math expressions with multiple terms
- String support
- Inequality expressions
- While loops or if statements

## Part 3 (5 pts)

Write at least 10 tests for your lexer. Make sure at least 2 of your tests test your new source code from pt. 2

## Part 4 - Code Demo and Explanation (10 pts)

This is the oral portion of the exam. You will record an explanation for your lexer which demonstrates its implementation and functionality. You don't have to show your face but you do have to record your voice (accommodations are available upon request). You should be sure to cover the following points in your discussion:

- Explain how your lexer works and supports the new token variants (those beyond HW2). 
- Demonstrate examples where it works correctly.
- What challenges did you face? How did you overcome them?

If you didn't finish the exam in is entirety, explain how you attempted to solve it and where you got stuck. This will get you at least some points. 

You can use Zoom to do this, [here is a link](https://support.zoom.us/hc/en-us/articles/360059781332-Getting-started-with-recording) to some instructions. You don't have to record your face, only your voice and the screen. Go through the answer and explain how you arrived there. Your goal with this question is to convince me you know what you are talking about, so I want you to do this without reading a script or written answer. Just go through line by line and explain what the program does. When you are done, upload your recording to your Lehigh Drive and add a link below. 

**⚠️IMPORTANT: Make sure you give blanket permission to the link holder to view the file**

🎥 Paste Recording Link(s) Here:

## Submission

Please submit your completed exam, which should include:

1. The lexer source code
2. Test cases demonstrating the correct lexing of the source code.
3. A recording link with permission to view granted to the link holder.

- Only files under version control in your forked assignment repository will be graded. Local files left untracked on your computer will not be considered.

- Only code committed *and pushed* prior to the time of grading will be accepted. Locally committed but unpushed code will not be considered.

- Your assignment will be graded according to the [Programming Assignment Grading Rubric](https://drive.google.com/open?id=1V0nBt3Rz6uFMZ9mIaFioLF-48DFX0VdkbgRUDM_eIFk).

Your submission should be organized, well-commented, and easy to understand. Remember to document any assumptions you made during the implementation process, as well as any limitations of your solution. Your final exam will be graded on the correctness, completeness, and clarity of your submission.

## Works Cited

List all sources used during the exam here.


