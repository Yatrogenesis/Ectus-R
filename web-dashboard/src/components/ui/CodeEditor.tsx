import React, { useRef, useEffect, useState } from 'react'
import * as monaco from 'monaco-editor'
import { useTheme } from '@/contexts/ThemeContext'
import { cn } from '@/lib/utils'

export interface CodeEditorProps {
  value: string
  onChange?: (value: string) => void
  language?: string
  theme?: 'light' | 'dark' | 'auto'
  height?: string | number
  width?: string | number
  readOnly?: boolean
  minimap?: boolean
  wordWrap?: 'on' | 'off' | 'wordWrapColumn' | 'bounded'
  fontSize?: number
  lineNumbers?: 'on' | 'off' | 'relative' | 'interval'
  folding?: boolean
  scrollBeyondLastLine?: boolean
  automaticLayout?: boolean
  tabSize?: number
  insertSpaces?: boolean
  className?: string
  onMount?: (editor: monaco.editor.IStandaloneCodeEditor) => void
  onFocus?: () => void
  onBlur?: () => void
}

export const CodeEditor: React.FC<CodeEditorProps> = ({
  value,
  onChange,
  language = 'javascript',
  theme = 'auto',
  height = '400px',
  width = '100%',
  readOnly = false,
  minimap = true,
  wordWrap = 'on',
  fontSize = 14,
  lineNumbers = 'on',
  folding = true,
  scrollBeyondLastLine = false,
  automaticLayout = true,
  tabSize = 2,
  insertSpaces = true,
  className,
  onMount,
  onFocus,
  onBlur,
}) => {
  const editorRef = useRef<HTMLDivElement>(null)
  const monacoRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null)
  const { resolvedTheme } = useTheme()
  const [isLoading, setIsLoading] = useState(true)

  const getTheme = () => {
    if (theme === 'auto') {
      return resolvedTheme === 'dark' ? 'vs-dark' : 'vs'
    }
    return theme === 'dark' ? 'vs-dark' : 'vs'
  }

  useEffect(() => {
    if (!editorRef.current) return

    // Configure Monaco Editor
    monaco.editor.defineTheme('aion-dark', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'comment', foreground: '6B7280', fontStyle: 'italic' },
        { token: 'keyword', foreground: '8B5CF6' },
        { token: 'string', foreground: '10B981' },
        { token: 'number', foreground: 'F59E0B' },
        { token: 'type', foreground: '3B82F6' },
        { token: 'function', foreground: 'EF4444' },
      ],
      colors: {
        'editor.background': '#1F2937',
        'editor.foreground': '#F9FAFB',
        'editorLineNumber.foreground': '#6B7280',
        'editorLineNumber.activeForeground': '#F9FAFB',
        'editor.selectionBackground': '#374151',
        'editor.lineHighlightBackground': '#374151',
      },
    })

    monaco.editor.defineTheme('aion-light', {
      base: 'vs',
      inherit: true,
      rules: [
        { token: 'comment', foreground: '6B7280', fontStyle: 'italic' },
        { token: 'keyword', foreground: '8B5CF6' },
        { token: 'string', foreground: '059669' },
        { token: 'number', foreground: 'D97706' },
        { token: 'type', foreground: '2563EB' },
        { token: 'function', foreground: 'DC2626' },
      ],
      colors: {
        'editor.background': '#FFFFFF',
        'editor.foreground': '#111827',
        'editorLineNumber.foreground': '#9CA3AF',
        'editorLineNumber.activeForeground': '#111827',
        'editor.selectionBackground': '#E5E7EB',
        'editor.lineHighlightBackground': '#F9FAFB',
      },
    })

    const editor = monaco.editor.create(editorRef.current, {
      value,
      language,
      theme: resolvedTheme === 'dark' ? 'aion-dark' : 'aion-light',
      readOnly,
      minimap: { enabled: minimap },
      wordWrap,
      fontSize,
      lineNumbers,
      folding,
      scrollBeyondLastLine,
      automaticLayout,
      tabSize,
      insertSpaces,
      smoothScrolling: true,
      cursorBlinking: 'smooth',
      cursorSmoothCaretAnimation: true,
      renderLineHighlight: 'gutter',
      renderWhitespace: 'selection',
      contextmenu: true,
      mouseWheelZoom: true,
      quickSuggestions: true,
      suggestOnTriggerCharacters: true,
      acceptSuggestionOnEnter: 'on',
      snippetSuggestions: 'top',
      formatOnPaste: true,
      formatOnType: true,
    })

    monacoRef.current = editor

    // Set up event listeners
    if (onChange) {
      editor.onDidChangeModelContent(() => {
        onChange(editor.getValue())
      })
    }

    if (onFocus) {
      editor.onDidFocusEditorText(onFocus)
    }

    if (onBlur) {
      editor.onDidBlurEditorText(onBlur)
    }

    // Configure language-specific settings
    if (language === 'typescript' || language === 'javascript') {
      monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
        target: monaco.languages.typescript.ScriptTarget.ES2020,
        allowNonTsExtensions: true,
        moduleResolution: monaco.languages.typescript.ModuleResolutionKind.NodeJs,
        module: monaco.languages.typescript.ModuleKind.CommonJS,
        noEmit: true,
        esModuleInterop: true,
        jsx: monaco.languages.typescript.JsxEmit.React,
        reactNamespace: 'React',
        allowJs: true,
        typeRoots: ['node_modules/@types'],
      })

      monaco.languages.typescript.typescriptDefaults.setDiagnosticsOptions({
        noSemanticValidation: false,
        noSyntaxValidation: false,
      })
    }

    setIsLoading(false)
    onMount?.(editor)

    return () => {
      editor?.dispose()
    }
  }, [])

  // Update theme when it changes
  useEffect(() => {
    if (monacoRef.current) {
      monaco.editor.setTheme(resolvedTheme === 'dark' ? 'aion-dark' : 'aion-light')
    }
  }, [resolvedTheme])

  // Update value when it changes externally
  useEffect(() => {
    if (monacoRef.current && value !== monacoRef.current.getValue()) {
      monacoRef.current.setValue(value)
    }
  }, [value])

  // Update language when it changes
  useEffect(() => {
    if (monacoRef.current) {
      const model = monacoRef.current.getModel()
      if (model) {
        monaco.editor.setModelLanguage(model, language)
      }
    }
  }, [language])

  return (
    <div className={cn('relative border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden', className)}>
      {isLoading && (
        <div className="absolute inset-0 flex items-center justify-center bg-gray-50 dark:bg-gray-800 z-10">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        </div>
      )}
      <div
        ref={editorRef}
        style={{ height, width }}
        className="w-full"
      />
    </div>
  )
}

export interface CodePreviewProps {
  code: string
  language: string
  title?: string
  showLineNumbers?: boolean
  highlightLines?: number[]
  className?: string
}

export const CodePreview: React.FC<CodePreviewProps> = ({
  code,
  language,
  title,
  showLineNumbers = true,
  highlightLines = [],
  className,
}) => {
  const { resolvedTheme } = useTheme()

  return (
    <div className={cn('bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden', className)}>
      {title && (
        <div className="px-4 py-2 bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600">
          <h4 className="text-sm font-medium text-gray-900 dark:text-white">{title}</h4>
        </div>
      )}
      <div className="relative">
        <pre className={cn(
          'p-4 overflow-x-auto text-sm',
          resolvedTheme === 'dark' ? 'bg-gray-900 text-gray-100' : 'bg-gray-50 text-gray-900'
        )}>
          <code className={`language-${language}`}>
            {code.split('\n').map((line, index) => (
              <div
                key={index}
                className={cn(
                  'block',
                  highlightLines.includes(index + 1) && 'bg-yellow-200 dark:bg-yellow-900/20 -mx-4 px-4'
                )}
              >
                {showLineNumbers && (
                  <span className="mr-4 text-gray-500 dark:text-gray-400 select-none">
                    {String(index + 1).padStart(2, ' ')}
                  </span>
                )}
                {line || ' '}
              </div>
            ))}
          </code>
        </pre>
      </div>
    </div>
  )
}