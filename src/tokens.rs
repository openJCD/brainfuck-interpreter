pub mod tokenizer {
    enum Tokens {
        AddToCell,
        SubtractFromCell,
        NextCell,
        PreviousCell,
        OpenLoop(u32),
        CloseLoop(u32),
        OutputCell,
        InputCell,
    }
}