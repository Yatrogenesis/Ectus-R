// AION-R Code Generation Page

import { useState } from 'react';
import { useMutation, useQuery } from '@tanstack/react-query';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';

// UI Components
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Separator } from '@/components/ui/separator';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';

// Icons
import {
  Code2,
  Settings,
  Wand2,
  Download,
  Eye,
  FileText,
  Lightbulb,
  AlertCircle,
  CheckCircle,
  Clock,
  Zap,
} from 'lucide-react';

// Code Editor
import Editor from '@monaco-editor/react';

// API
import { apiClient } from '@/lib/api';
import { useToast } from '@/hooks/use-toast';

// Types
interface CodeGenerationRequest {
  requirements: string;
  language: string;
  framework?: string;
  architecture: string;
  optimization_level: string;
  constraints?: {
    include_tests: boolean;
    include_docs: boolean;
    max_file_size?: number;
    performance_requirements?: {
      max_latency_ms?: number;
      min_throughput_rps?: number;
    };
    security_requirements?: {
      encryption_required: boolean;
      compliance_standards: string[];
    };
  };
}

interface CodeGenerationResult {
  id: string;
  status: string;
  generated_files_count: number;
  total_lines_of_code: number;
  documentation_url: string;
  download_url: string;
  preview: {
    main_file: string;
    structure: Array<{
      path: string;
      language: string;
      size_bytes: number;
      purpose: string;
    }>;
  };
  suggestions: string[];
  estimated_time_saved_hours: number;
}

// Form Schema
const formSchema = z.object({
  requirements: z.string().min(10, 'Requirements must be at least 10 characters'),
  language: z.string().min(1, 'Please select a language'),
  framework: z.string().optional(),
  architecture: z.string().min(1, 'Please select an architecture'),
  optimization_level: z.string().min(1, 'Please select optimization level'),
  include_tests: z.boolean().default(true),
  include_docs: z.boolean().default(true),
  encryption_required: z.boolean().default(false),
  compliance_standards: z.array(z.string()).default([]),
});

type FormData = z.infer<typeof formSchema>;

const LANGUAGES = [
  { value: 'rust', label: 'Rust' },
  { value: 'python', label: 'Python' },
  { value: 'javascript', label: 'JavaScript' },
  { value: 'typescript', label: 'TypeScript' },
  { value: 'go', label: 'Go' },
  { value: 'java', label: 'Java' },
  { value: 'csharp', label: 'C#' },
  { value: 'cpp', label: 'C++' },
];

const ARCHITECTURES = [
  { value: 'microservices', label: 'Microservices' },
  { value: 'monolithic', label: 'Monolithic' },
  { value: 'serverless', label: 'Serverless' },
  { value: 'layered', label: 'Layered' },
  { value: 'hexagonal', label: 'Hexagonal' },
  { value: 'mvc', label: 'MVC' },
  { value: 'clean', label: 'Clean Architecture' },
];

const OPTIMIZATION_LEVELS = [
  { value: 'none', label: 'None', description: 'No optimization' },
  { value: 'basic', label: 'Basic', description: 'Basic optimizations' },
  { value: 'balanced', label: 'Balanced', description: 'Balance between speed and size' },
  { value: 'performance', label: 'Performance', description: 'Optimize for speed' },
  { value: 'size', label: 'Size', description: 'Optimize for size' },
  { value: 'maximum', label: 'Maximum', description: 'All optimizations' },
];

const FRAMEWORKS = {
  rust: ['axum', 'warp', 'actix-web', 'rocket'],
  python: ['fastapi', 'django', 'flask', 'starlette'],
  javascript: ['express', 'koa', 'hapi', 'nest'],
  typescript: ['express', 'nest', 'koa', 'fastify'],
  go: ['gin', 'echo', 'fiber', 'chi'],
  java: ['spring-boot', 'quarkus', 'micronaut', 'helidon'],
  csharp: ['asp.net-core', 'minimal-api', 'blazor'],
  cpp: ['crow', 'pistache', 'drogon'],
};

export function CodeGeneration() {
  const [result, setResult] = useState<CodeGenerationResult | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [progress, setProgress] = useState(0);
  const { toast } = useToast();

  const form = useForm<FormData>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      requirements: '',
      language: 'rust',
      architecture: 'layered',
      optimization_level: 'balanced',
      include_tests: true,
      include_docs: true,
      encryption_required: false,
      compliance_standards: [],
    },
  });

  const selectedLanguage = form.watch('language');
  const selectedFrameworks = FRAMEWORKS[selectedLanguage as keyof typeof FRAMEWORKS] || [];

  // Generate Code Mutation
  const generateMutation = useMutation({
    mutationFn: async (data: FormData) => {
      const request: CodeGenerationRequest = {
        requirements: data.requirements,
        language: data.language,
        framework: data.framework,
        architecture: data.architecture,
        optimization_level: data.optimization_level,
        constraints: {
          include_tests: data.include_tests,
          include_docs: data.include_docs,
          security_requirements: {
            encryption_required: data.encryption_required,
            compliance_standards: data.compliance_standards,
          },
        },
      };

      return apiClient.post<CodeGenerationResult>('/api/v1/code/generate', request);
    },
    onSuccess: (data) => {
      setResult(data);
      setIsGenerating(false);
      toast({
        title: 'Code Generation Complete!',
        description: `Generated ${data.generated_files_count} files with ${data.total_lines_of_code} lines of code.`,
      });
    },
    onError: (error: any) => {
      setIsGenerating(false);
      toast({
        title: 'Generation Failed',
        description: error.response?.data?.message || 'An error occurred during code generation.',
        variant: 'destructive',
      });
    },
  });

  const onSubmit = (data: FormData) => {
    setIsGenerating(true);
    setProgress(0);
    setResult(null);

    // Simulate progress
    const progressInterval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 90) {
          clearInterval(progressInterval);
          return prev;
        }
        return prev + Math.random() * 10;
      });
    }, 1000);

    generateMutation.mutate(data);
  };

  const handleDownload = async () => {
    if (!result) return;

    try {
      const response = await apiClient.get(`/api/v1/code/download/${result.id}`, {
        responseType: 'blob',
      });

      const blob = new Blob([response.data]);
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `generation-${result.id}.zip`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);

      toast({
        title: 'Download Started',
        description: 'Your generated code is being downloaded.',
      });
    } catch (error) {
      toast({
        title: 'Download Failed',
        description: 'Failed to download the generated code.',
        variant: 'destructive',
      });
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Code Generation</h1>
          <p className="text-muted-foreground">
            Generate production-ready code from your requirements using AI
          </p>
        </div>
      </div>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Form Section */}
        <div className="lg:col-span-2">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Wand2 className="h-5 w-5" />
                <span>Generation Configuration</span>
              </CardTitle>
              <CardDescription>
                Describe your requirements and configure generation parameters
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
                  {/* Requirements */}
                  <FormField
                    control={form.control}
                    name="requirements"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Requirements</FormLabel>
                        <FormControl>
                          <Textarea
                            placeholder="Describe what you want to build. Be as detailed as possible. For example: 'Create a REST API for user management with authentication, CRUD operations, and input validation...'"
                            className="min-h-[120px]"
                            {...field}
                          />
                        </FormControl>
                        <FormDescription>
                          Provide detailed requirements for better code generation
                        </FormDescription>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  <div className="grid gap-4 md:grid-cols-2">
                    {/* Language */}
                    <FormField
                      control={form.control}
                      name="language"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Programming Language</FormLabel>
                          <Select onValueChange={field.onChange} defaultValue={field.value}>
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue placeholder="Select language" />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              {LANGUAGES.map((lang) => (
                                <SelectItem key={lang.value} value={lang.value}>
                                  {lang.label}
                                </SelectItem>
                              ))}
                            </SelectContent>
                          </Select>
                          <FormMessage />
                        </FormItem>
                      )}
                    />

                    {/* Framework */}
                    <FormField
                      control={form.control}
                      name="framework"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Framework (Optional)</FormLabel>
                          <Select onValueChange={field.onChange} value={field.value}>
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue placeholder="Select framework" />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              {selectedFrameworks.map((framework) => (
                                <SelectItem key={framework} value={framework}>
                                  {framework}
                                </SelectItem>
                              ))}
                            </SelectContent>
                          </Select>
                          <FormMessage />
                        </FormItem>
                      )}
                    />

                    {/* Architecture */}
                    <FormField
                      control={form.control}
                      name="architecture"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Architecture Pattern</FormLabel>
                          <Select onValueChange={field.onChange} defaultValue={field.value}>
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue placeholder="Select architecture" />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              {ARCHITECTURES.map((arch) => (
                                <SelectItem key={arch.value} value={arch.value}>
                                  {arch.label}
                                </SelectItem>
                              ))}
                            </SelectContent>
                          </Select>
                          <FormMessage />
                        </FormItem>
                      )}
                    />

                    {/* Optimization Level */}
                    <FormField
                      control={form.control}
                      name="optimization_level"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Optimization Level</FormLabel>
                          <Select onValueChange={field.onChange} defaultValue={field.value}>
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue placeholder="Select optimization" />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              {OPTIMIZATION_LEVELS.map((opt) => (
                                <SelectItem key={opt.value} value={opt.value}>
                                  <div>
                                    <div>{opt.label}</div>
                                    <div className="text-xs text-muted-foreground">
                                      {opt.description}
                                    </div>
                                  </div>
                                </SelectItem>
                              ))}
                            </SelectContent>
                          </Select>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>

                  {/* Options */}
                  <div className="space-y-4">
                    <Label className="text-base font-semibold">Generation Options</Label>
                    <div className="grid gap-4 md:grid-cols-2">
                      <FormField
                        control={form.control}
                        name="include_tests"
                        render={({ field }) => (
                          <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                            <div className="space-y-0.5">
                              <FormLabel className="text-base">Include Tests</FormLabel>
                              <FormDescription>
                                Generate unit and integration tests
                              </FormDescription>
                            </div>
                            <FormControl>
                              <Switch
                                checked={field.value}
                                onCheckedChange={field.onChange}
                              />
                            </FormControl>
                          </FormItem>
                        )}
                      />

                      <FormField
                        control={form.control}
                        name="include_docs"
                        render={({ field }) => (
                          <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                            <div className="space-y-0.5">
                              <FormLabel className="text-base">Include Documentation</FormLabel>
                              <FormDescription>
                                Generate README and API docs
                              </FormDescription>
                            </div>
                            <FormControl>
                              <Switch
                                checked={field.value}
                                onCheckedChange={field.onChange}
                              />
                            </FormControl>
                          </FormItem>
                        )}
                      />

                      <FormField
                        control={form.control}
                        name="encryption_required"
                        render={({ field }) => (
                          <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                            <div className="space-y-0.5">
                              <FormLabel className="text-base">Encryption Required</FormLabel>
                              <FormDescription>
                                Include encryption for sensitive data
                              </FormDescription>
                            </div>
                            <FormControl>
                              <Switch
                                checked={field.value}
                                onCheckedChange={field.onChange}
                              />
                            </FormControl>
                          </FormItem>
                        )}
                      />
                    </div>
                  </div>

                  <Button
                    type="submit"
                    className="w-full"
                    disabled={isGenerating}
                    size="lg"
                  >
                    {isGenerating ? (
                      <>
                        <Zap className="mr-2 h-4 w-4 animate-spin" />
                        Generating Code...
                      </>
                    ) : (
                      <>
                        <Code2 className="mr-2 h-4 w-4" />
                        Generate Code
                      </>
                    )}
                  </Button>
                </form>
              </Form>
            </CardContent>
          </Card>
        </div>

        {/* Results Section */}
        <div className="space-y-6">
          {/* Progress */}
          {isGenerating && (
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center space-x-2">
                  <Clock className="h-5 w-5 animate-spin" />
                  <span>Generating...</span>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  <Progress value={progress} />
                  <p className="text-sm text-muted-foreground">
                    Analyzing requirements and generating code...
                  </p>
                </div>
              </CardContent>
            </Card>
          )}

          {/* Results */}
          {result && (
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center space-x-2">
                  <CheckCircle className="h-5 w-5 text-green-500" />
                  <span>Generation Complete</span>
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid gap-2 text-sm">
                  <div className="flex justify-between">
                    <span>Files Generated:</span>
                    <Badge variant="secondary">{result.generated_files_count}</Badge>
                  </div>
                  <div className="flex justify-between">
                    <span>Lines of Code:</span>
                    <Badge variant="secondary">{result.total_lines_of_code.toLocaleString()}</Badge>
                  </div>
                  <div className="flex justify-between">
                    <span>Time Saved:</span>
                    <Badge variant="secondary">{result.estimated_time_saved_hours}h</Badge>
                  </div>
                </div>

                <Separator />

                <div className="space-y-2">
                  <Button onClick={handleDownload} className="w-full">
                    <Download className="mr-2 h-4 w-4" />
                    Download Code
                  </Button>
                  <Button variant="outline" className="w-full">
                    <Eye className="mr-2 h-4 w-4" />
                    View Preview
                  </Button>
                  <Button variant="outline" className="w-full">
                    <FileText className="mr-2 h-4 w-4" />
                    View Documentation
                  </Button>
                </div>

                {/* Suggestions */}
                {result.suggestions.length > 0 && (
                  <>
                    <Separator />
                    <div className="space-y-2">
                      <h4 className="text-sm font-semibold flex items-center space-x-2">
                        <Lightbulb className="h-4 w-4" />
                        <span>Suggestions</span>
                      </h4>
                      <div className="space-y-1">
                        {result.suggestions.map((suggestion, index) => (
                          <p key={index} className="text-xs text-muted-foreground">
                            • {suggestion}
                          </p>
                        ))}
                      </div>
                    </div>
                  </>
                )}
              </CardContent>
            </Card>
          )}

          {/* File Structure Preview */}
          {result && (
            <Card>
              <CardHeader>
                <CardTitle>File Structure</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2 text-sm">
                  {result.preview.structure.map((file, index) => (
                    <div key={index} className="flex items-center justify-between p-2 rounded border">
                      <div>
                        <p className="font-mono text-xs">{file.path}</p>
                        <p className="text-xs text-muted-foreground">{file.purpose}</p>
                      </div>
                      <Badge variant="outline" className="text-xs">
                        {file.language}
                      </Badge>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}
        </div>
      </div>
    </div>
  );
}