// For more information see https://aka.ms/fsharp-console-apps
open System

printfn "Hello from F#, please enter your name: "

let name = Console.ReadLine()
printf "Hello %s!\n" name

printf "Enter any key to exit: "
Console.ReadKey() |> ignore


