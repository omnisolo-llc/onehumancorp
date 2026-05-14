import 'package:flutter/material.dart';
import 'models.dart';
import 'engine.dart';

/// The StorefrontEditor provides a mobile-first interface for business owners
/// to visually construct and modify their storefronts.
///
/// It operates on a list of `Block` models, allowing users to:
/// 1. Reorder blocks vertically via an intuitive drag-and-drop interface.
/// 2. Edit the textual or structural content of individual blocks via a clean,
///    bottom-sheet-based modal form.
/// 3. Toggle between an "Edit" mode (showing the block structure and drag handles)
///    and a "Preview" mode (utilizing `StorefrontEngine` to show the final result).
/// 4. Publish their changes to make the draft layout live.
///
/// This component emphasizes simplicity and the "grandmother test", ensuring that
/// even the most non-technical users can assemble a beautiful site without needing
/// to understand HTML, CSS, or complex layout concepts.
class StorefrontEditor extends StatefulWidget {
  /// The initial configuration of blocks to populate the editor.
  final List<Block> initialBlocks;

  /// Callback triggered whenever a block's content is updated or the order of blocks changes.
  /// The updated list of blocks is passed as an argument.
  final Function(List<Block>) onSave;

  /// Callback triggered when the user explicitly taps the "Publish" button to finalize their layout.
  final Function() onPublish;

  /// Constructs a `StorefrontEditor`.
  const StorefrontEditor({
    Key? key,
    required this.initialBlocks,
    required this.onSave,
    required this.onPublish
  }) : super(key: key);

  @override
  _StorefrontEditorState createState() => _StorefrontEditorState();
}

class _StorefrontEditorState extends State<StorefrontEditor> {
  /// The local, mutable state representing the current draft configuration.
  late List<Block> blocks;

  /// Toggles the UI between structural editing (ReorderableListView) and accurate preview (StorefrontEngine).
  bool isPreviewMode = false;

  @override
  void initState() {
    super.initState();
    // Initialize the local state from the widget parameters.
    blocks = List.from(widget.initialBlocks);
  }

  /// Opens a highly accessible modal bottom sheet to edit the properties of a specific block.
  ///
  /// The form fields presented dynamically adapt based on the `blockType`. For example,
  /// a `HeroBlock` provides fields for `headline` and `subtitle`.
  /// We use `showModalBottomSheet` because it is an established, thumb-friendly mobile UI pattern
  /// that keeps the user grounded in context.
  void _editBlock(int index) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true, // Allows the sheet to resize when the keyboard appears.
      backgroundColor: Colors.transparent, // Required for custom rounded corners.
      builder: (context) {
        return Container(
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.9), // Subtle glassmorphism effect.
            borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
          ),
          padding: EdgeInsets.only(
            // Dynamically pad the bottom to ensure the keyboard doesn't obscure the input fields.
            bottom: MediaQuery.of(context).viewInsets.bottom,
            left: 20,
            right: 20,
            top: 20,
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min, // Wrap content tightly.
            children: [
              Text('Edit ${blocks[index].blockType}', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),

              // Dynamic form fields based on block type.
              // Currently implements HeroBlock editing as a proof of concept.
              if (blocks[index].blockType == 'HeroBlock')
                TextField(
                  decoration: InputDecoration(labelText: 'Headline'),
                  controller: TextEditingController(text: blocks[index].content['headline']),
                  onChanged: (val) {
                    setState(() {
                      blocks[index].content['headline'] = val;
                    });
                  },
                ),
              SizedBox(height: 20),
              ElevatedButton(
                onPressed: () {
                  // Propagate the changes upwards using the callback.
                  widget.onSave(blocks);
                  Navigator.pop(context); // Dismiss the sheet.
                },
                child: Text('Save Block'),
              ),
              SizedBox(height: 20),
            ],
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Storefront Editor'),
        actions: [
          IconButton(
            icon: Icon(isPreviewMode ? Icons.edit : Icons.preview),
            tooltip: isPreviewMode ? 'Switch to Edit Mode' : 'Switch to Preview Mode',
            onPressed: () {
              setState(() {
                isPreviewMode = !isPreviewMode;
              });
            },
          ),
          ElevatedButton(
            onPressed: widget.onPublish,
            child: Text('Publish'),
          ),
        ],
      ),
      // Conditionally render the exact preview or the editable structural list.
      body: isPreviewMode
          ? StorefrontEngine(blocks: blocks)
          : ReorderableListView(
              onReorder: (oldIndex, newIndex) {
                setState(() {
                  // ReorderableListView's newIndex logic requires this adjustment.
                  if (newIndex > oldIndex) {
                    newIndex -= 1;
                  }
                  final item = blocks.removeAt(oldIndex);
                  blocks.insert(newIndex, item);

                  // Immediately persist the order change.
                  widget.onSave(blocks);
                });
              },
              children: blocks.asMap().entries.map((entry) {
                int index = entry.key;
                Block block = entry.value;
                return ListTile(
                  // A unique key is strictly required by ReorderableListView to track items during drag operations.
                  key: ValueKey(block.id),
                  title: Text(block.blockType),
                  subtitle: Text(block.content.toString(), maxLines: 1, overflow: TextOverflow.ellipsis),
                  trailing: Icon(Icons.drag_handle),
                  onTap: () => _editBlock(index),
                );
              }).toList(),
            ),
    );
  }
}
