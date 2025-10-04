import 'package:flutter/material.dart';
import 'package:flutter_code_editor/flutter_code_editor.dart';
import 'package:flutter_highlight/themes/monokai-sublime.dart';
import 'package:highlight/languages/1c.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Two TextFields Example',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final CodeController _codeController = CodeController(
    text: ''' ''',
    language: lang1C,
  );

  void _handleSubmit() {
    final code = _codeController.text;
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Submitted code:\n$code')),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Code Editor Example')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            // Row: editor left, instructions right
            Expanded(
              child: Row(
                children: [
                  // Code editor
                  Expanded(
                    flex: 2,
                    child: CodeEditorWidget(controller: _codeController),
                  ),
                  const SizedBox(width: 16),
                  // Instructions
                  Expanded(
                    flex: 1,
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: const [
                        Text(
                          'Instructions',
                          style: TextStyle(
                              fontSize: 20, fontWeight: FontWeight.bold),
                        ),
                        SizedBox(height: 12),
                        Text(
                            '1. Write a Dart function in the editor.\n'
                            '2. Make it print "Hello, Flutter!".\n'
                            '3. Click Submit when ready.'),
                      ],
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 20),
            // Submit button
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: _handleSubmit,
                child: const Text('Submit'),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// Separate widget just for the code editor
class CodeEditorWidget extends StatelessWidget {
  final CodeController controller;
  const CodeEditorWidget({super.key, required this.controller});

  @override
  Widget build(BuildContext context) {
    return CodeTheme(
      data: CodeThemeData(styles: monokaiSublimeTheme),
      child: SingleChildScrollView(
        child: CodeField(
          controller: controller,
          textStyle: const TextStyle(fontFamily: 'monospace', fontSize: 14),
        ),
      ),
    );
  }
}
