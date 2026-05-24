mod syntax;

mod prototype {
    use aeris_ui::syntax;

    syntax! {
        {
            pub NestedA;
            NestedA -> "foo";
            NestedA -> NestedB;
            {
                pub NestedB;
                NestedB -> "bar";
            }
        }
    }
}
