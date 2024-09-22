# Java Agent


We can run java programs with a java agent. This agent will get called before the java program starts running.
on this call it will get a chance to set up some hooks so that when certain events happen in the java program, the agent will get called.
For example, the agent can get called when a class is loaded, when a method is entered, when a method is exited, when an object is created, etc.


This project is for learning purposes. It implments a simple java agent in rust.
