class SearchPalette extends StatelessWidget {
  const SearchPalette({super.key});

  @override
  Widget build(BuildContext context) {
    return Dialog(
      alignment: Alignment.topCenter,
      child: Container(
        width: 500,
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              autofocus: true,
              decoration: const InputDecoration(hintText: "Go To... (Reports, Vouchers, etc.)"),
              onChanged: (val) async {
                // Call Rust Engine for real-time search suggestions
                final suggestions = await getSearchSuggestions(input: val);
                print("Suggestions from Rust: $suggestions");
              },
            ),
            // Results would be rendered here in a ListView
          ],
        ),
      ),
    );
  }
}